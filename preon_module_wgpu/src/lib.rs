use std::mem::size_of;

use preon_engine::{
    events::{PreonEvent, PreonEventEmitter},
    rendering::{PreonRenderPass, PreonRenderer, PreonShape},
};
use wgpu::util::DeviceExt;
use winit::{dpi::PhysicalSize, window::Window};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 2],
}

impl Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x2,
            }],
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct TransformationUniform {
    transformation: [f32; 2],
}

impl TransformationUniform {
    fn new(size_x: f32, size_y: f32) -> Self {
        let mut new_self = Self {
            transformation: [1.0, 1.0],
        };

        new_self.resize(size_x, size_y);
        new_self
    }

    fn resize(&mut self, size_x: f32, size_y: f32) {
        self.transformation = [2.0f32 / size_x, 2.0f32 / size_y];
    }

    fn raw(&self) -> [f32; 2] {
        self.transformation
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct RectInstance {
    radius: [f32; 4],
    dimensions: [f32; 4],
    color: [f32; 4],
}

impl RectInstance {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: size_of::<RectInstance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

const RECT_VERTICES: &[Vertex] = &[
    Vertex {
        position: [0.0, 0.0],
    },
    Vertex {
        position: [0.0, -1.0],
    },
    Vertex {
        position: [1.0, -1.0],
    },
    Vertex {
        position: [1.0, 0.0],
    },
];

const RECT_INDICES: &[u16] = &[0, 1, 2, 3, 0, 2, 0];

pub mod preon {
    use std::time::{Duration, Instant};

    use preon_engine::{
        components::PreonCustomComponentStack,
        events::{PreonEvent, PreonEventEmitter, PreonUserEvent},
        rendering::PreonRenderer,
        types::PreonVector,
        PreonEngine,
    };
    use winit::{
        dpi::LogicalSize,
        event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    };

    use crate::PreonRendererWGPU;

    /// Initialize winit and run your app, this is sufficient for simple apps, if you plan on building something advanced you should consider starting it yourself so you can have a little more control over individual events.
    pub fn run<T, F>(mut engine: PreonEngine<T>, mut callback: F)
    where
        T: PreonCustomComponentStack + 'static,
        F: FnMut(PreonEvent) + 'static,
    {
        engine.start();
        env_logger::init();

        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_min_inner_size(LogicalSize::new(20, 2))
            .with_max_inner_size(LogicalSize::new(1000000, 1000000))
            .with_visible(false)
            .build(&event_loop)
            .unwrap();

        let mut wgpu = PreonRendererWGPU::new(&window);
        wgpu.start();

        window.set_visible(true);

        let (mut ctrl, mut shift, mut logo, mut alt) = (false, false, false, false);
        let mut user_events = PreonEventEmitter::new();
        user_events.push(PreonUserEvent::WindowOpened);

        event_loop.run(move |event, _, control_flow| match event {
            Event::RedrawRequested(_) => {
                user_events.flip();

                if engine.update(&mut user_events) {
                    engine.events.pull(|e| callback(e));

                    wgpu.update(&mut engine.events);
                    wgpu.render(&mut engine.render_pass);
                }
            }
            Event::RedrawEventsCleared => *control_flow = ControlFlow::Wait,
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                WindowEvent::CloseRequested => {
                    callback(PreonEvent::WindowClosed);
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::Resized(physical_size) => {
                    wgpu.resize(*physical_size);
                    user_events.push(PreonUserEvent::WindowResized(PreonVector::new(
                        physical_size.width,
                        physical_size.height,
                    )));
                    *control_flow = ControlFlow::WaitUntil(Instant::now() + Duration::from_secs_f32(0.1f32));
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    wgpu.resize(**new_inner_size);
                    user_events.push(PreonUserEvent::WindowResized(PreonVector::new(
                        new_inner_size.width,
                        new_inner_size.height,
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
                        callback(PreonEvent::WindowClosed);
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
    render_pipeline: wgpu::RenderPipeline,
    rect_vertex_buffer: wgpu::Buffer,
    rect_index_buffer: wgpu::Buffer,
    rect_transform_buffer: wgpu::Buffer,
    rect_transform_uniform: TransformationUniform,
    rect_transform_bind_group: wgpu::BindGroup,
    rect_instances: Vec<RectInstance>,
    rect_instance_buffer: wgpu::Buffer,
}

impl PreonRendererWGPU {
    pub fn new(window: &Window) -> Self {
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
                present_mode: wgpu::PresentMode::Fifo,
            };
            surface.configure(&device, &config);

            let rect_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Rect Vertex Buffer"),
                contents: bytemuck::cast_slice(RECT_VERTICES),
                usage: wgpu::BufferUsages::VERTEX,
            });

            let rect_index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Rect Index Buffer"),
                contents: bytemuck::cast_slice(RECT_INDICES),
                usage: wgpu::BufferUsages::INDEX,
            });

            let rect_transform_uniform = TransformationUniform::new(1.0, 1.0);

            let rect_transform_buffer =
                device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Rect Transform Uniform"),
                    contents: bytemuck::cast_slice(&rect_transform_uniform.raw()),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                });

            let rect_transform_bind_group_layout =
                device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("Rect Transform Bind Group Layout"),
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: wgpu::BufferSize::new(size_of::<[f32; 2]>() as _),
                        },
                        count: None,
                    }],
                });

            let rect_transform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Rect Transform Bind Group"),
                layout: &rect_transform_bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: rect_transform_buffer.as_entire_binding(),
                }],
            });

            let rect_instances: Vec<RectInstance> = Vec::new();
            let rect_instance_buffer =
                device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Rect Instances"),
                    contents: bytemuck::cast_slice(rect_instances.as_slice()),
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                });

            let rect_vert_shader = wgpu::include_wgsl!("shaders/rect_shader.vert.wgsl");
            let rect_vert_module = device.create_shader_module(&rect_vert_shader);
            let rect_frag_shader = wgpu::include_wgsl!("shaders/rect_shader.frag.wgsl");
            let rect_frag_module = device.create_shader_module(&rect_frag_shader);

            let render_pipeline_layout =
                device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[&rect_transform_bind_group_layout],
                    push_constant_ranges: &[],
                });

            let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &rect_vert_module,
                    entry_point: "main",
                    buffers: &[Vertex::desc(), RectInstance::desc()],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &rect_frag_module,
                    entry_point: "main",
                    targets: &[config.format.into()],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    polygon_mode: wgpu::PolygonMode::Fill,
                    clamp_depth: false,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
            });

            (
                surface,
                device,
                queue,
                config,
                size,
                render_pipeline,
                rect_vertex_buffer,
                rect_index_buffer,
                rect_transform_buffer,
                rect_transform_uniform,
                rect_transform_bind_group,
                rect_instance_buffer,
            )
        };

        let (
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            rect_vertex_buffer,
            rect_index_buffer,
            rect_transform_buffer,
            rect_transform_uniform,
            rect_transform_bind_group,
            rect_instance_buffer,
        ) = pollster::block_on(task);

        Self {
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            rect_vertex_buffer,
            rect_index_buffer,
            rect_transform_buffer,
            rect_transform_uniform,
            rect_transform_bind_group,
            rect_instances: Vec::new(),
            rect_instance_buffer,
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);

            self.rect_transform_uniform
                .resize(self.size.width as f32, self.size.height as f32);
            self.queue.write_buffer(
                &self.rect_transform_buffer,
                0,
                bytemuck::cast_slice(&self.rect_transform_uniform.raw()),
            );
        }
    }
}

impl PreonRenderer for PreonRendererWGPU {
    fn start(&mut self) {}

    fn update(&mut self, _events: &mut PreonEventEmitter<PreonEvent>) {}

    fn render(&mut self, pass: &mut PreonRenderPass) {
        let previous_size = self.rect_instances.len();

        self.rect_instances = Vec::with_capacity(pass.len());
        pass.pull(|s| match s {
            PreonShape::Rect {
                color,
                position,
                size,
            } => {
                self.rect_instances.push(RectInstance {
                    radius: [0.0, 0.0, 0.0, 0.0],
                    dimensions: [
                        position.x as f32,
                        position.y as f32,
                        size.x as f32,
                        size.y as f32,
                    ],
                    color: {
                        let (r, g, b, a) = color.into_f32_tuple();
                        [r, g, b, a]
                    },
                });
            }
        });

        if previous_size != self.rect_instances.len() {
            self.rect_instance_buffer =
                self.device
                    .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("New Rect Instance Buffer"),
                        contents: bytemuck::cast_slice(self.rect_instances.as_slice()),
                        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                    });
        } else {
            self.queue.write_buffer(
                &self.rect_instance_buffer,
                0,
                bytemuck::cast_slice(self.rect_instances.as_slice()),
            );
        }

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
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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
                    depth_stencil_attachment: None,
                });

                render_pass.set_vertex_buffer(1, self.rect_instance_buffer.slice(..));
                render_pass.set_pipeline(&self.render_pipeline);
                render_pass.set_bind_group(0, &self.rect_transform_bind_group, &[]);
                render_pass.set_vertex_buffer(0, self.rect_vertex_buffer.slice(..));
                render_pass
                    .set_index_buffer(self.rect_index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                render_pass.draw_indexed(0..7, 0, 0..self.rect_instances.len() as u32);
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
