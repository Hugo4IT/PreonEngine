use std::mem::size_of;

use wgpu::util::DeviceExt;
use winit::dpi::PhysicalSize;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct TransformationUniform {
    transformation: [f32; 2],
}

impl TransformationUniform {
    fn new(size_x: f32, size_y: f32) -> Self {
        Self {
            transformation: [2.0f32 / size_x, 2.0f32 / size_y],
        }
    }

    fn resize(&mut self, size_x: f32, size_y: f32) {
        self.transformation = [2.0f32 / size_x, 2.0f32 / size_y];
    }

    fn raw(&self) -> [f32; 2] {
        self.transformation
    }
}

pub struct Transform {
    buffer: wgpu::Buffer,
    uniform: TransformationUniform,
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
}

impl Transform {
    pub fn new(device: &wgpu::Device) -> Self {
        let uniform = TransformationUniform::new(1.0, 1.0);

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Rect Transform Uniform"),
            contents: bytemuck::cast_slice(&uniform.raw()),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Rect Transform Bind Group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });

        Self {
            buffer,
            uniform,
            bind_group,
            bind_group_layout,
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>, queue: &wgpu::Queue) {
        self.uniform
            .resize(new_size.width as f32, new_size.height as f32);
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&self.uniform.raw()));
    }
}
