use log::info;
use preon_engine::{
    rendering::{PreonRenderPass, PreonShape, PreonStaticRenderData},
    types::PreonVector,
};
use wgpu::util::DeviceExt;
use winit::dpi::PhysicalSize;

use crate::{shapes::text::TextShape, texture::Texture};

use self::{
    rect::RectShape,
    static_texture::StaticTextureShape,
    transform::Transform,
    vertex::{RECT_INDICES, RECT_VERTICES},
};

mod rect;
mod static_texture;
mod text;
mod transform;
mod vertex;

pub struct ShapeManager {
    transform: Transform,

    depth_texture: Texture,

    rect: RectShape,
    text: TextShape,
    static_texture: StaticTextureShape,

    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
}

impl ShapeManager {
    pub fn new(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        queue: &wgpu::Queue,
        static_render_data: &PreonStaticRenderData,
    ) -> Self {
        info!("Creating depth buffer...");
        let depth_texture = Texture::new_depth(device, config);

        info!("Initializing buffers...");
        let transform = Transform::new(device, config.width as f32, config.height as f32);
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Rect Vertex Buffer"),
            contents: bytemuck::cast_slice(RECT_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Rect Index Buffer"),
            contents: bytemuck::cast_slice(RECT_INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        info!("Init RectShape...");
        let rect = RectShape::new(device, config, &transform.bind_group_layout);

        info!("Init TextShape...");
        let text = TextShape::new(device, static_render_data.fonts, config.format);

        info!("Init StaticTextureShape...");
        let static_texture = StaticTextureShape::new(
            device,
            config,
            queue,
            &transform.bind_group_layout,
            static_render_data.textures,
        );

        Self {
            transform,
            depth_texture,
            rect,
            text,
            static_texture,
            vertex_buffer,
            index_buffer,
        }
    }

    /// Translate PreonRenderPass to instanced wgpu::RenderPass instructions, and apply z_index
    pub fn build(&mut self, pass: &PreonRenderPass, device: &wgpu::Device, queue: &wgpu::Queue) {
        let z_step = 1.0 / (pass.len() + 1) as f32;

        self.rect.instance_buffer.begin();
        self.static_texture.instance_buffer.begin();

        let mut z_index: f32 = 1.0 - z_step;

        pass.pull(|shape| {
            match shape {
                PreonShape::Rect { .. } => self.rect.build(shape, z_index),
                PreonShape::StaticTexture { .. } => self.static_texture.build(shape, z_index),
                PreonShape::Text { .. } => self.text.build(shape, z_index),
            }

            z_index -= z_step;
        });

        self.static_texture.instance_buffer.end(device, queue);
        self.rect.instance_buffer.end(device, queue);
    }

    /// Execute instanced wgpu render calls with the built wgpu::RenderPass instructions from ShapeManager::build();
    pub fn render(
        &mut self,
        screen_size: PreonVector<i32>,
        view: &wgpu::TextureView,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
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
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });

            render_pass.set_bind_group(0, &self.transform.bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

            render_pass = self.rect.render(render_pass);
            self.static_texture.render(render_pass);
        }

        self.text.render(device, encoder, view, screen_size);
    }

    /// Correct transformation after resizing
    pub fn resize(
        &mut self,
        new_size: PhysicalSize<u32>,
        queue: &wgpu::Queue,
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
    ) {
        self.depth_texture = Texture::new_depth(device, config);
        self.transform.resize(new_size, queue);
    }
}
