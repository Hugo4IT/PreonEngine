use log::info;
use preon_engine::rendering::{PreonRenderPass, PreonShape};
use wgpu::util::DeviceExt;
use winit::dpi::PhysicalSize;

use self::{
    rect::RectShape,
    static_texture::StaticTextureShape,
    transform::Transform,
    vertex::{RECT_INDICES, RECT_VERTICES},
};

mod rect;
mod static_texture;
mod static_text;
mod transform;
mod vertex;

pub struct ShapeManager {
    transform: Transform,

    rect: RectShape,
    static_texture: StaticTextureShape,

    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
}

impl ShapeManager {
    pub fn new(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        queue: &wgpu::Queue,
        static_textures: &[&[u8]],
    ) -> Self {
        info!("Initializing buffers...");
        let transform = Transform::new(device);
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

        info!("Init StaticTextureShape...");
        let static_texture = StaticTextureShape::new(
            device,
            config,
            queue,
            &transform.bind_group_layout,
            static_textures,
        );

        Self {
            transform,
            rect,
            static_texture,
            vertex_buffer,
            index_buffer,
        }
    }

    /// Translate PreonRenderPass to instanced wgpu::RenderPass instructions, and apply z_index
    pub fn build(&mut self, pass: &PreonRenderPass, device: &wgpu::Device, queue: &wgpu::Queue) {
        let z_step = 1.0 / pass.len() as f32; // French DejaVu

        self.rect.instance_buffer.begin();
        self.static_texture.instance_buffer.begin();

        let mut z_index: f32 = 1.0 - z_step;

        pass.pull(|shape| {
            match shape {
                PreonShape::Rect { .. } => self.rect.build(shape, z_index),
                PreonShape::StaticTexture { .. } => self.static_texture.build(shape, z_index),
                PreonShape::Text { .. } => {}
            }

            z_index -= z_step;
        });

        self.static_texture.instance_buffer.end(device, queue);
        self.rect.instance_buffer.end(device, queue);
    }

    /// Execute instanced wgpu render calls with the built wgpu::RenderPass instructions from ShapeManager::build();
    pub fn render<'a>(&'a self, mut render_pass: wgpu::RenderPass<'a>) {
        render_pass.set_bind_group(0, &self.transform.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

        render_pass = self.rect.render(render_pass);
        self.static_texture.render(render_pass);
    }

    /// Correct transformation after resizing
    pub fn resize(&mut self, new_size: PhysicalSize<u32>, queue: &wgpu::Queue) {
        self.transform.resize(new_size, queue);
    }
}
