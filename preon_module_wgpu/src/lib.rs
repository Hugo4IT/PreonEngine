use log::info;
use preon_engine::{
    components::PreonCustomComponentStack, rendering::PreonRenderPass, types::PreonVector,
    PreonEngine,
};
use shapes::ShapeManager;
use winit::{dpi::PhysicalSize, window::Window};

mod instancing;
mod shapes;
mod texture;

pub mod preon {
    use preon_engine::{
        components::{PreonComponent, PreonCustomComponentStack},
        events::{PreonEvent, PreonEventEmitter, PreonUserEvent},
        types::PreonVector,
        PreonEngine,
    };
    use winit::{
        event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    };

    use crate::PreonRendererWGPU;

    /// Initialize winit and run your app, this is sufficient for simple apps, if you plan on building something advanced you should consider starting it yourself so you can have a little more control over individual events.
    pub fn run<T, F>(mut engine: PreonEngine<T>, mut callback: F)
    where
        T: PreonCustomComponentStack + 'static,
        F: FnMut(&mut PreonComponent<T>, PreonEvent, &mut PreonEventEmitter<PreonUserEvent>)
            + 'static,
    {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_visible(false)
            .build(&event_loop)
            .unwrap();

        let mut renderer = PreonRendererWGPU::new(&window, &engine);
        window.set_visible(true);

        let (mut ctrl, mut shift, mut logo, mut alt) = (false, false, false, false);
        let mut user_events = PreonEventEmitter::new();
        user_events.push(PreonUserEvent::WindowOpened);
        user_events.push(PreonUserEvent::WindowResized(PreonVector {
            x: window.inner_size().width,
            y: window.inner_size().height,
        }));

        let mut await_close = false;

        event_loop.run(move |event, _, control_flow| match event {
            Event::RedrawRequested(_) => {
                user_events.flip();

                if engine.update(&user_events) {
                    let tree = engine.tree.as_mut().unwrap();

                    engine.events.pull(|event| {
                        callback(tree, event, &mut user_events);
                    });

                    if renderer.render(&mut engine.render_pass) {
                        *control_flow = ControlFlow::Exit;
                    }
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
                    renderer.resize(*physical_size);
                    user_events.push(PreonUserEvent::WindowResized(PreonVector::new(
                        physical_size.width,
                        physical_size.height,
                    )));

                    *control_flow = ControlFlow::Wait;
                    window.request_redraw();
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    renderer.resize(**new_inner_size);
                    user_events.push(PreonUserEvent::WindowResized(PreonVector::new(
                        new_inner_size.width,
                        new_inner_size.height,
                    )));

                    *control_flow = ControlFlow::Wait;
                    window.request_redraw();
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
                        user_events.push(PreonUserEvent::WindowClosed);
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
                        user_events.push(PreonUserEvent::WindowClosed);
                        *control_flow = ControlFlow::Exit;
                    }
                }
                _ => (),
            },
            _ => (),
        });
    }
}

pub struct PreonRendererWGPU {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: PhysicalSize<u32>,
    shape_manager: ShapeManager,
}

impl PreonRendererWGPU {
    pub fn new<T: PreonCustomComponentStack>(window: &Window, engine: &PreonEngine<T>) -> Self {
        #[cfg(feature = "android")]
        {
            info!("Detected android platform (--features android), waiting for NativeWindow...");

            loop {
                match ndk_glue::native_window().as_ref() {
                    Some(_) => break,
                    None => (),
                }
            }

            info!("Got NativeWindow.");
        }

        info!("Initializing Surface...");

        let size = window.inner_size();
        let backend = wgpu::util::backend_bits_from_env().unwrap_or_else(wgpu::Backends::all);
        let instance = wgpu::Instance::new(backend);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = pollster::block_on(wgpu::util::initialize_adapter_from_env_or_default(
            &instance,
            backend,
            Some(&surface),
        ))
        .expect("No suitable graphics adapters found.");

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits:
                    wgpu::Limits::downlevel_webgl2_defaults().using_resolution(adapter.limits()),
                label: None,
            },
            None,
        ))
        .unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        info!("Init ShapeManager...");
        let shape_manager = ShapeManager::new(&device, &config, &queue, &engine.static_render_data);

        info!("WGPU Initialized!");

        Self {
            surface,
            device,
            queue,
            config,
            size,
            shape_manager,
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        info!(
            "Resize requested for new_size: {}x{}",
            new_size.width, new_size.height
        );

        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.shape_manager
                .resize(new_size, &self.queue, &self.device, &self.config);

            info!("Accepted!");
        } else {
            info!("Requested size was too small.")
        }
    }

    fn render(&mut self, pass: &mut PreonRenderPass) -> bool {
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
                self.shape_manager.render(
                    PreonVector::new(self.config.width as i32, self.config.height as i32),
                    &view,
                    &self.device,
                    &mut encoder,
                );
            }

            self.queue.submit(Some(encoder.finish()));
            output.present();

            Ok(())
        };

        match res {
            Ok(_) => {}
            Err(wgpu::SurfaceError::Lost) => self.resize(self.size),
            Err(wgpu::SurfaceError::OutOfMemory) => return true,
            Err(e) => eprintln!("{:?}", e),
        }

        false
    }
}
