use std::mem::size_of;

use preon_engine::rendering::PreonShape;

use crate::{instancing::{BufferLayout, InstanceBuffer}, shapes::vertex::Vertex, texture::{Texture, TextureSheet}};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TextureInstance {
    z_index: f32,
    dimensions: [f32; 4],
    uv_dimensions: [f32; 4],
}

impl BufferLayout for TextureInstance {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: size_of::<TextureInstance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32,
                },
                wgpu::VertexAttribute {
                    offset: size_of::<f32>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: size_of::<[f32; 5]>() as wgpu::BufferAddress,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

pub struct StaticTextureShape {
    pub pipeline: wgpu::RenderPipeline,
    pub instance_buffer: InstanceBuffer<TextureInstance>,
    pub sheet: TextureSheet,
}

impl StaticTextureShape {
    pub fn new(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        queue: &wgpu::Queue,
        transform_bind_group_layout: &wgpu::BindGroupLayout,
        textures: &[&[u8]]
    ) -> Self {
        let sheet =
            TextureSheet::from_images(textures, device, queue);
        let instance_buffer = InstanceBuffer::new(device);

        let vert_shader = wgpu::include_wgsl!("../shaders/texture_shader.vert.wgsl");
        let vert_module = device.create_shader_module(&vert_shader);
        let frag_shader = wgpu::include_wgsl!("../shaders/texture_shader.frag.wgsl");
        let frag_module = device.create_shader_module(&frag_shader);

        let pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[
                    transform_bind_group_layout,
                    &sheet.bind_group_layout,
                ],
                push_constant_ranges: &[],
            });

        let pipeline =
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &vert_module,
                    entry_point: "main",
                    buffers: &[Vertex::desc(), TextureInstance::desc()],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &frag_module,
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
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: Texture::DEPTH_FORMAT,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::Less,
                    stencil: wgpu::StencilState::default(),
                    bias: wgpu::DepthBiasState::default(),
                }),
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
            });

        Self { pipeline, instance_buffer, sheet }
    }

    pub fn build(&mut self, shape: PreonShape, z_index: f32) {
        if let PreonShape::StaticTexture {
            position,
            size,
            index,
        } = shape {
            self.instance_buffer.push(TextureInstance {
                z_index,
                dimensions: [
                    position.x as f32,
                    position.y as f32,
                    size.x as f32,
                    size.y as f32,
                ],
                uv_dimensions: self.sheet.indices[index],
            });
        }
    }

    pub fn render<'a>(&'a self, mut render_pass: wgpu::RenderPass<'a>) -> wgpu::RenderPass<'a> {
        render_pass.set_vertex_buffer(1, self.instance_buffer.get());
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(1, &self.sheet.bind_group, &[]);
        render_pass.draw_indexed(0..7, 0, 0..self.instance_buffer.len() as u32);

        render_pass
    }
}
