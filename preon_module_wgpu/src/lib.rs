use preon_engine::{
    components::PreonCustomComponentStack,
    rendering::PreonRenderPass,
    PreonEngine,
};
use shapes::ShapeManager;
use texture::Texture;
use winit::{dpi::PhysicalSize, window::Window};

mod shapes;
mod texture;
mod instancing;

pub mod preon {
    use std::time::{Duration, Instant};

    use preon_engine::{
        components::{PreonComponent, PreonCustomComponentStack},
        events::{PreonEvent, PreonEventEmitter, PreonUserEvent},
        types::PreonVector,
        PreonEngine,
    };
    use winit::{event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder};

    use crate::PreonRendererWGPU;

    /// Initialize winit and run your app, this is sufficient for simple apps, if you plan on building something advanced you should consider starting it yourself so you can have a little more control over individual events.
    pub fn run<T, F>(mut engine: PreonEngine<T>, mut callback: F)
    where
        T: PreonCustomComponentStack + 'static,
        F: FnMut(&mut PreonComponent<T>, PreonEvent, &mut PreonEventEmitter<PreonUserEvent>)
            + 'static,
    {
        env_logger::init();

        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_visible(false)
            .build(&event_loop)
            .unwrap();

        let mut wgpu = PreonRendererWGPU::new(&window, &engine);

        window.set_visible(true);

        let (mut ctrl, mut shift, mut logo, mut alt) = (false, false, false, false);
        let mut user_events = PreonEventEmitter::new();
        user_events.push(PreonUserEvent::WindowOpened);

        let mut await_close = false;

        event_loop.run(move |event, _, control_flow| match event {
            Event::RedrawRequested(_) => {
                user_events.flip();

                if engine.update(&user_events) {
                    let mut tree = engine.tree.take().unwrap();

                    engine.events.pull(|event| {
                        callback(&mut tree, event, &mut user_events);
                    });

                    engine.tree = Some(tree);
                    wgpu.render(&mut engine.render_pass);
                }

                if await_close {
                    *control_flow = ControlFlow::Exit;
                }
            }
            Event::RedrawEventsCleared => {
                if user_events.buffer_len() > 0 {
                    window.request_redraw();
                } else {
                    *control_flow = ControlFlow::Wait;
                }
            }
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                WindowEvent::CloseRequested => {
                    user_events.push(PreonUserEvent::WindowClosed);
                    await_close = true;
                    window.request_redraw();
                }
                WindowEvent::Resized(physical_size) => {
                    wgpu.resize(*physical_size);
                    user_events.push(PreonUserEvent::WindowResized(PreonVector::new(
                        physical_size.width,
                        physical_size.height,
                    )));
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    wgpu.resize(**new_inner_size);
                    user_events.push(PreonUserEvent::WindowResized(PreonVector::new(
                        new_inner_size.width,
                        new_inner_size.height,
                    )));
                }
                WindowEvent::CursorMoved { position, .. } => {
                    user_events.push(PreonUserEvent::MouseMove(PreonVector::new(
                        position.x as i32,
                        position.y as i32,
                    )));
                }
                WindowEvent::ModifiersChanged(modifier) => {
                    ctrl = modifier.ctrl();
                    shift = modifier.shift();
                    logo = modifier.logo();
                    alt = modifier.alt();
                }
                #[cfg(target_os = "linux")]
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Q),
                            ..
                        },
                    ..
                } => {
                    if ctrl && !shift && !logo && !alt {
                        user_events.push(PreonUserEvent::WindowClosed);
                        *control_flow = ControlFlow::Exit;
                    }
                }
                #[cfg(target_os = "macos")]
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Q),
                            ..
                        },
                    ..
                } => {
                    if logo && !shift && !ctrl && !alt {
                        callback(&PreonEvent::WindowClosed);
                        *control_flow = ControlFlow::Exit;
                    }
                }
                #[cfg(target_os = "windows")]
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::F4),
                            ..
                        },
                    ..
                } => {
                    if alt && !ctrl && !shift && !logo {
                        callback(&PreonEvent::WindowClosed);
                        *control_flow = ControlFlow::Exit;
                    }
                }
                _ => {}
            },
            _ => {}
        });
    }
}

pub struct PreonRendererWGPU {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: PhysicalSize<u32>,
    depth_texture: Texture,
    shape_manager: ShapeManager,
}

impl PreonRendererWGPU {
    pub fn new<T: PreonCustomComponentStack>(window: &Window, engine: &PreonEngine<T>) -> Self {
        let task = async {
            let size = window.inner_size();
            let backend = wgpu::util::backend_bits_from_env().unwrap_or_else(wgpu::Backends::all);
            let instance = wgpu::Instance::new(backend);
            let surface = unsafe { instance.create_surface(window) };
            let adapter = wgpu::util::initialize_adapter_from_env_or_default(
                &instance,
                backend,
                Some(&surface),
            )
            .await
            .expect("No suitable graphics adapters found.");

            let (device, queue) = adapter
                .request_device(
                    &wgpu::DeviceDescriptor {
                        features: wgpu::Features::empty(),
                        limits: wgpu::Limits::downlevel_webgl2_defaults()
                            .using_resolution(adapter.limits()),
                        label: None,
                    },
                    None,
                )
                .await
                .unwrap();

            let config = wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: surface.get_preferred_format(&adapter).unwrap(),
                width: size.width,
                height: size.height,
                present_mode: wgpu::PresentMode::Immediate,
            };
            surface.configure(&device, &config);

            let depth_texture = Texture::new_depth(&device, &config);
            let shape_manager = ShapeManager::new(
                &device,
                &config,
                &queue,
                &engine.static_render_data.textures
            );

            (
                surface,
                device,
                queue,
                config,
                size,
                depth_texture,
                shape_manager
            )
        };

        let (
            surface,
            device,
            queue,
            config,
            size,
            depth_texture,
            shape_manager
        ) = pollster::block_on(task);

        Self {
            surface,
            device,
            queue,
            config,
            size,
            depth_texture,
            shape_manager
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);

            self.depth_texture = Texture::new_depth(&self.device, &self.config);
            self.shape_manager.resize(new_size, &self.queue);
        }
    }

    fn render(&mut self, pass: &mut PreonRenderPass) {
        self.shape_manager.build(pass, &self.device, &self.queue);

        let res: Result<(), wgpu::SurfaceError> = {
            let output = self.surface.get_current_texture().unwrap();
            let view = output
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());
            let mut encoder = self
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

            {
                let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.006022458,
                                g: 0.006022458,
                                b: 0.006022458,
                                a: 1.0,
                            }),
                            store: true,
                        },
                    }],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &self.depth_texture.view,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.0),
                            store: true,
                        }),
                        stencil_ops: None,
                    }),
                });

                self.shape_manager.render(render_pass);
            }

            self.queue.submit(Some(encoder.finish()));
            output.present();

            Ok(())
        };

        match res {
            Ok(_) => {}
            Err(wgpu::SurfaceError::Lost) => self.resize(self.size),
            Err(wgpu::SurfaceError::OutOfMemory) => panic!(
                "Out of memory, press F to pay respects to the pregnancy test running this app"
            ),
            Err(e) => eprintln!("{:?}", e),
        }
    }
}
