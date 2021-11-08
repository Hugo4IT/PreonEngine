use std::mem::size_of;

use font_kit::source::SystemSource;
use preon_engine::{PreonEngine, components::PreonCustomComponentStack, rendering::{PreonRenderPass, PreonShape}};
use wgpu::util::DeviceExt;
use winit::{dpi::PhysicalSize, window::Window};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2]
}

impl Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2
                }
            ],
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

pub struct PreonAtlasMeta {}
impl sheep::Format for PreonAtlasMeta {
    type Data = Vec<[f32; 4]>;
    type Options = u8;

    fn encode(
        dimensions: (u32, u32),
        sprites: &[sheep::SpriteAnchor],
        _options: Self::Options,
    ) -> Self::Data {
        let mut _sprites = sprites.to_vec();
        _sprites.sort_by(|left, right| {
            left.id.cmp(&right.id)
        });

        let mut data = Vec::with_capacity(_sprites.len());
        _sprites.iter().for_each(|anchor| {
            data.push([
                (anchor.position.0 as f32) / (dimensions.0 as f32),
                (anchor.position.1 as f32) / (dimensions.1 as f32),
                (anchor.dimensions.0 as f32) / (dimensions.0 as f32),
                (anchor.dimensions.1 as f32) / (dimensions.1 as f32),
            ])
        });
        data
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct TextureInstance {
    dimensions: [f32; 4],
    uv_dimensions: [f32; 4]
}

impl TextureInstance {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: size_of::<TextureInstance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x4
                }
            ],
        }
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
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

const RECT_VERTICES: &[Vertex] = &[
    Vertex {
        position: [0.0, 0.0],
        tex_coords: [0.0, 0.0],
    },
    Vertex {
        position: [0.0, -1.0],
        tex_coords: [0.0, 1.0],
    },
    Vertex {
        position: [1.0, -1.0],
        tex_coords: [1.0, 1.0],
    },
    Vertex {
        position: [1.0, 0.0],
        tex_coords: [1.0, 0.0],
    },
];

const RECT_INDICES: &[u16] = &[0, 1, 2, 3, 0, 2, 0];

pub mod preon {
    use std::time::{Duration, Instant};

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
                    *control_flow =
                        ControlFlow::WaitUntil(Instant::now() + Duration::from_secs_f32(0.05f32));
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
    transform_buffer: wgpu::Buffer,
    transform_uniform: TransformationUniform,
    transform_bind_group: wgpu::BindGroup,

    // Rect
    rect_pipeline: wgpu::RenderPipeline,
    rect_vertex_buffer: wgpu::Buffer,
    rect_index_buffer: wgpu::Buffer,
    rect_instances: Vec<RectInstance>,
    rect_instance_buffer: wgpu::Buffer,

    // StaticTexture
    static_texture_pipeline: wgpu::RenderPipeline,
    static_texture_bind_group: wgpu::BindGroup,
    static_texture_instances: Vec<TextureInstance>,
    static_texture_instance_buffer: wgpu::Buffer,
    static_texture_uv_dimensions: Vec<[f32; 4]>,

    // StaticText
    static_text_offset: usize,
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

            let transform_uniform = TransformationUniform::new(1.0, 1.0);

            let transform_buffer =
                device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Rect Transform Uniform"),
                    contents: bytemuck::cast_slice(&transform_uniform.raw()),
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

            let transform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Rect Transform Bind Group"),
                layout: &rect_transform_bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: transform_buffer.as_entire_binding(),
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

            let rect_pipeline_layout =
                device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[&rect_transform_bind_group_layout],
                    push_constant_ranges: &[],
                });

            let rect_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&rect_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &rect_vert_module,
                    entry_point: "main",
                    buffers: &[Vertex::desc(), RectInstance::desc()],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &rect_frag_module,
                    entry_point: "main",
                    targets: &[wgpu::ColorTargetState {
                        format: config.format,
                        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                        write_mask: wgpu::ColorWrites::ALL,
                    }],
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

            // Textures

            let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            });

            let texture_bind_group_layout = device.create_bind_group_layout(
                &wgpu::BindGroupLayoutDescriptor {
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Texture {
                                multisampled: false,
                                view_dimension: wgpu::TextureViewDimension::D2,
                                sample_type: wgpu::TextureSampleType::Float {
                                    filterable: true
                                },
                            },
                            count: None
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Sampler {
                                comparison: false,
                                filtering: true,
                            },
                            count: None
                        }
                    ],
                    label: Some("BindGroupLayout for textures")
                }
            );

            let mut textures: Vec<sheep::InputSprite> = Vec::new();

            for buffer in engine.static_render_data.textures.iter() {
                let image = image::load_from_memory(*buffer).unwrap();

                use image::GenericImageView;
                let dimensions = image.dimensions();

                textures.push(sheep::InputSprite {
                    bytes: image
                        .as_rgba8()
                        .unwrap()
                        .pixels()
                        .flat_map(|p| p.0.iter().map(|b|*b))
                        .collect::<Vec<u8>>(),
                    dimensions
                });
            }

            let sprite_sheet = sheep::pack::<sheep::MaxrectsPacker>(textures, 4, Default::default());
            let sprite_sheet = sprite_sheet.into_iter().next().unwrap();
            let sheet_size = wgpu::Extent3d {
                width: sprite_sheet.dimensions.0,
                height: sprite_sheet.dimensions.1,
                depth_or_array_layers: 1
            };

            let static_texture_uv_dimensions = sheep::encode::<PreonAtlasMeta>(&sprite_sheet, 0);

            image::save_buffer(
                "cache.bmp",
                sprite_sheet.bytes.as_slice(),
                sprite_sheet.dimensions.0,
                sprite_sheet.dimensions.1,
                image::ColorType::Rgba8
            ).unwrap();

            let texture = device.create_texture(
                &wgpu::TextureDescriptor {
                    size: sheet_size,
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format: wgpu::TextureFormat::Rgba8UnormSrgb,
                    usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                    label: Some("StaticTexture atlas")
                }
            );

            queue.write_texture(
                wgpu::ImageCopyTexture {
                    texture: &texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                sprite_sheet.bytes.as_slice(),
                wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: std::num::NonZeroU32::new(4 * sprite_sheet.dimensions.0),
                    rows_per_image: std::num::NonZeroU32::new(sprite_sheet.dimensions.1)
                },
                sheet_size
            );

            let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

            let static_texture_bind_group = device.create_bind_group(
                &wgpu::BindGroupDescriptor {
                    layout: &texture_bind_group_layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::TextureView(&texture_view)
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::Sampler(&sampler)
                        }
                    ],
                    label: Some("BindGroup for StaticTexture atlas")
                }
            );

            let static_texture_instances: Vec<TextureInstance> = Vec::new();
            let static_texture_instance_buffer =
                device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Texture Instances"),
                    contents: bytemuck::cast_slice(static_texture_instances.as_slice()),
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                });

            let texture_vert_shader = wgpu::include_wgsl!("shaders/texture_shader.vert.wgsl");
            let texture_vert_module = device.create_shader_module(&texture_vert_shader);
            let texture_frag_shader = wgpu::include_wgsl!("shaders/texture_shader.frag.wgsl");
            let texture_frag_module = device.create_shader_module(&texture_frag_shader);

            let texture_pipeline_layout =
                device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[&rect_transform_bind_group_layout, &texture_bind_group_layout],
                    push_constant_ranges: &[],
                });

            let static_texture_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&texture_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &texture_vert_module,
                    entry_point: "main",
                    buffers: &[Vertex::desc(), TextureInstance::desc()],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &texture_frag_module,
                    entry_point: "main",
                    targets: &[wgpu::ColorTargetState {
                        format: config.format,
                        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                        write_mask: wgpu::ColorWrites::ALL,
                    }],
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
                transform_buffer,
                transform_uniform,
                transform_bind_group,
                rect_pipeline,
                rect_vertex_buffer,
                rect_index_buffer,
                rect_instance_buffer,
                static_texture_pipeline,
                static_texture_bind_group,
                static_texture_instances,
                static_texture_instance_buffer,
                static_texture_uv_dimensions,
                engine.static_render_data.textures.len()
            )
        };

        let (
            surface,
            device,
            queue,
            config,
            size,
            transform_buffer,
            transform_uniform,
            transform_bind_group,
            rect_pipeline,
            rect_vertex_buffer,
            rect_index_buffer,
            rect_instance_buffer,
            static_texture_pipeline,
            static_texture_bind_group,
            static_texture_instances,
            static_texture_instance_buffer,
            static_texture_uv_dimensions,
            static_text_offset
        ) = pollster::block_on(task);

        Self {
            surface,
            device,
            queue,
            config,
            size,
            transform_uniform,
            transform_bind_group,
            rect_pipeline,
            rect_vertex_buffer,
            rect_index_buffer,
            transform_buffer,
            rect_instances: Vec::new(),
            rect_instance_buffer,
            static_texture_pipeline,
            static_texture_bind_group,
            static_texture_instances,
            static_texture_instance_buffer,
            static_texture_uv_dimensions,
            static_text_offset
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);

            self.transform_uniform
                .resize(self.size.width as f32, self.size.height as f32);
            self.queue.write_buffer(
                &self.transform_buffer,
                0,
                bytemuck::cast_slice(&self.transform_uniform.raw()),
            );
        }
    }

    fn render(&mut self, pass: &mut PreonRenderPass) {
        let previous_rect_count = self.rect_instances.len();
        let previous_texture_count = self.static_texture_instances.len();

        self.rect_instances.clear();
        self.static_texture_instances.clear();

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
            },
            PreonShape::StaticTexture {
                position,
                size,
                index,
            } => {
                self.static_texture_instances.push(TextureInstance {
                    dimensions: [
                        position.x as f32,
                        position.y as f32,
                        size.x as f32,
                        size.y as f32,
                    ],
                    uv_dimensions: self.static_texture_uv_dimensions[index]
                });
            },
            PreonShape::StaticText { // TODO: Text PreonShape Implementation
                position,
                size,
                index,
            } => {
                self.static_texture_instances.push(TextureInstance {
                    dimensions: [
                        position.x as f32,
                        position.y as f32,
                        size.x as f32,
                        size.y as f32
                    ],
                    uv_dimensions: self.static_texture_uv_dimensions[self.static_text_offset + index]
                });
            }
        });

        if previous_rect_count != self.rect_instances.len() {
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

        if previous_texture_count != self.static_texture_instances.len() {
            self.static_texture_instance_buffer =
                self.device
                    .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("New Texture Instance Buffer"),
                        contents: bytemuck::cast_slice(self.static_texture_instances.as_slice()),
                        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                    });
        } else {
            self.queue.write_buffer(
                &self.static_texture_instance_buffer,
                0,
                bytemuck::cast_slice(self.static_texture_instances.as_slice()),
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

                // Rect
                render_pass.set_vertex_buffer(1, self.rect_instance_buffer.slice(..));
                render_pass.set_pipeline(&self.rect_pipeline);
                render_pass.set_bind_group(0, &self.transform_bind_group, &[]);
                render_pass.set_vertex_buffer(0, self.rect_vertex_buffer.slice(..));
                render_pass
                    .set_index_buffer(self.rect_index_buffer.slice(..), wgpu::IndexFormat::Uint16);

                render_pass.draw_indexed(0..7, 0, 0..self.rect_instances.len() as u32);

                // Texture
                render_pass.set_vertex_buffer(1, self.static_texture_instance_buffer.slice(..));
                render_pass.set_pipeline(&self.static_texture_pipeline);
                render_pass.set_bind_group(0, &self.transform_bind_group, &[]);
                render_pass.set_bind_group(1, &self.static_texture_bind_group, &[]);
                render_pass.set_vertex_buffer(0, self.rect_vertex_buffer.slice(..));
                render_pass
                    .set_index_buffer(self.rect_index_buffer.slice(..), wgpu::IndexFormat::Uint16);

                render_pass.draw_indexed(0..7, 0, 0..self.static_texture_instances.len() as u32);
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
