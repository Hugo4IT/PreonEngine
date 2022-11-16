use std::mem::size_of;

use log::info;
use preon_engine::rendering::PreonShape;

use crate::{
    instancing::{BufferLayout, InstanceBuffer},
    shapes::vertex::Vertex,
    texture::{Texture, TextureSheet},
};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RectInstance {
    z_index: f32,
    rect: [f32; 4],
    radius: [f32; 4],
    color: [f32; 4],
    uv_cutout: [f32; 4],
}

impl BufferLayout for RectInstance {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: size_of::<RectInstance>() as wgpu::BufferAddress,
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
                wgpu::VertexAttribute {
                    offset: size_of::<[f32; 9]>() as wgpu::BufferAddress,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: size_of::<[f32; 13]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

pub struct RectShape {
    pub pipeline: wgpu::RenderPipeline,
    pub instance_buffer: InstanceBuffer<RectInstance>,
    pub sheet: TextureSheet,
}

impl RectShape {
    pub fn new(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        queue: &wgpu::Queue,
        transform_bind_group_layout: &wgpu::BindGroupLayout,
        textures: &Vec<Vec<u8>>,
    ) -> Self {
        info!("Init TextureSheet...");
        let sheet =
            TextureSheet::from_images(textures, device, queue, String::from("static_textures"));

        let instance_buffer = InstanceBuffer::new(device);

        info!("Compiling shaders...");
        let shader_module =
            device.create_shader_module(wgpu::include_wgsl!("../shaders/component_shader.wgsl"));

        info!("Creating render pipeline...");
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[transform_bind_group_layout, &sheet.bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader_module,
                entry_point: "vert_main",
                buffers: &[Vertex::desc(), RectInstance::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader_module,
                entry_point: "frag_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
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
            multiview: None,
        });

        Self {
            pipeline,
            instance_buffer,
            sheet,
        }
    }

    pub fn build(&mut self, shape: PreonShape, z_index: f32) {
        if let PreonShape::Rect {
            position,
            size,
            color,
            index,
            radius,
        } = shape
        {
            self.instance_buffer.push(RectInstance {
                z_index,
                rect: [
                    position.x as f32,
                    position.y as f32,
                    size.x as f32,
                    size.y as f32,
                ],
                radius: [
                    radius.top_left,
                    radius.top_right,
                    radius.bottom_right,
                    radius.bottom_left
                ],
                color: {
                    let (r, g, b, a) = color.into_f32_tuple();
                    [r, g, b, a]
                },
                uv_cutout: match index {
                    Some(index) => self.sheet.indices[index],
                    None => [-1.0, 0.0, 0.0, 0.0],
                }
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
