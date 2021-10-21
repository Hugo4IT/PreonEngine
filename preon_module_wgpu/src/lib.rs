use std::iter;

use preon_engine::{PreonRenderer, types::PreonColor};
use winit::{dpi::PhysicalSize, event::WindowEvent, window::Window};

pub mod preon {
    use preon_engine::{PreonEngine, PreonRenderer, components::PreonComponentStack};
    use winit::{event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder};

    use crate::PreonRendererWGPU;

    /// Initialize winit and run your app, this is sufficient for simple apps, if you plan on building something advanced you should consider starting it yourself so you can have a little more control over individual events.
    pub fn run<T: PreonComponentStack + 'static>(mut engine: PreonEngine<T>) {
        env_logger::init();
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();
        let mut wgpu = PreonRendererWGPU::new(&window);

        let (mut ctrl, mut shift, mut logo, mut alt) = (false, false, false, false);
    
        event_loop.run(move |event, _, control_flow| match event {
            Event::RedrawRequested(_) => {
                wgpu.update(&mut engine.events);
                wgpu.render(&mut engine.render_pass);
            },
            Event::MainEventsCleared => window.request_redraw(),
            Event::WindowEvent {
                ref event,
                window_id
            } if window_id == window.id() => if !wgpu.winit_event(event) {
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => wgpu.resize(*physical_size),
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => wgpu.resize(**new_inner_size),
                    WindowEvent::ModifiersChanged(modifier) => {
                        ctrl = modifier.ctrl();
                        shift = modifier.shift();
                        logo = modifier.logo();
                        alt = modifier.alt();
                    }
                    #[cfg(target_os = "linux")]
                    WindowEvent::KeyboardInput {
                        input: KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Q),
                            ..
                        },
                        ..
                    } => if ctrl && !shift && !logo && !alt {
                        *control_flow = ControlFlow::Exit;
                    },
                    #[cfg(target_os = "macos")]
                    WindowEvent::KeyboardInput {
                        input: KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Q),
                            ..
                        },
                        ..
                    } => if logo && !shift && !ctrl && !alt {
                        *control_flow = ControlFlow::Exit;
                    },
                    #[cfg(target_os = "windows")]
                    WindowEvent::KeyboardInput {
                        input: KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::F4),
                            ..
                        },
                        ..
                    } => if alt && !ctrl && !shift && !logo {
                        *control_flow = ControlFlow::Exit;
                    },
                    _ => {},
                }
            },
            _ => {},
        });
    }
}

pub struct PreonRendererWGPU {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: PhysicalSize<u32>,
}

impl PreonRendererWGPU {
    pub fn new(window: &Window) -> Self {
        let task = async {
            let size = window.inner_size();
            let instance = wgpu::Instance::new(wgpu::Backends::all());
            let surface = unsafe { instance.create_surface(window) };
            let adapter = instance.request_adapter(
                &wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::default(),
                    compatible_surface: Some(&surface),
                    force_fallback_adapter: false,
                },
            ).await.unwrap();

            let (device, queue) = adapter.request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            ).await.unwrap();

            let config = wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: surface.get_preferred_format(&adapter).unwrap(),
                width: size.width,
                height: size.height,
                present_mode: wgpu::PresentMode::Fifo,
            };
            surface.configure(&device, &config);

            (surface, device, queue, config, size)
        };

        let (surface, device, queue, config, size) = pollster::block_on(task);

        Self {
            surface, device, queue, config, size
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn winit_event(&mut self, event: &WindowEvent) -> bool {
        false
    }
}

impl PreonRenderer for PreonRendererWGPU {
    fn start(&mut self) {
        todo!()
    }

    fn update(&mut self, events: &mut preon_engine::events::PreonEventEmitter) -> bool {
        false
    }

    fn render(&mut self, render_pass: &mut preon_engine::rendering::PreonRenderPass) {
        let res: Result<(), wgpu::SurfaceError> = {
            let output = self.surface.get_current_texture().unwrap();
            let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
            let mut encoder = self.device.create_command_encoder(
                &wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                }
            );
    
            let (r, g, b, a) = PreonColor::from_hex("#171717").into_f64_tuple();
    
            {
                let _render_pass = encoder.begin_render_pass(
                    &wgpu::RenderPassDescriptor {
                        label: Some("Render Pass"),
                        color_attachments: &[
                            wgpu::RenderPassColorAttachment {
                                view: &view,
                                resolve_target: None,
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Clear(
                                        wgpu::Color { r, g, b, a }
                                    ),
                                    store: true,
                                }
                            }
                        ],
                        depth_stencil_attachment: None,
                    }
                );
            }
            
            self.queue.submit(iter::once(encoder.finish()));
            output.present();

            Ok(())
        };

        match res {
            Ok(_) => {},
            Err(wgpu::SurfaceError::Lost) => self.resize(self.size),
            Err(wgpu::SurfaceError::OutOfMemory) => panic!("Out of memory, press F to pay respects to the pregnancy test running this app"),
            Err(e) => eprintln!("{:?}", e),
        }
    }
}