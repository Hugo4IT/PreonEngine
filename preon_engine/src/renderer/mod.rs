use log::info;
use winit::{dpi::PhysicalSize, window::Window};

use self::rect::RectRenderer;

pub mod instance;
pub mod rect;
pub mod vertex;

pub struct Renderer {
    config: wgpu::SurfaceConfiguration,
    surface: wgpu::Surface,

    device: wgpu::Device,
    queue: wgpu::Queue,

    size_bind_group: wgpu::BindGroup,
    size_buffer: wgpu::Buffer,
    pub rect: RectRenderer,
}

impl Renderer {
    pub fn new(window: &Window) -> Renderer {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(
            wgpu::util::backend_bits_from_env().unwrap_or(wgpu::Backends::all()),
        );
        let surface = unsafe { instance.create_surface(window) };
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            compatible_surface: Some(&surface),
            power_preference: wgpu::PowerPreference::LowPower,
            force_fallback_adapter: false,
        }))
        .unwrap();

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("Render Device"),
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::downlevel_webgl2_defaults(),
            },
            None,
        ))
        .unwrap();

        let config = wgpu::SurfaceConfiguration {
            present_mode: wgpu::PresentMode::Fifo,
            format: surface
                .get_preferred_format(&adapter)
                .unwrap_or(wgpu::TextureFormat::Rgba8Unorm),
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            height: size.height,
            width: size.width,
        };
        surface.configure(&device, &config);

        use wgpu::util::DeviceExt;
        let size_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Screen Size Uniform Buffer"),
            contents: bytemuck::cast_slice(&[config.width as f32, config.height as f32]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let size_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Screen Size Uniform Buffer Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let size_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Screen Size Uniform Buffer Bind Group"),
            layout: &size_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: size_buffer.as_entire_binding(),
            }],
        });

        let rect_renderer = RectRenderer::new(&device, &config, &size_bind_group_layout);

        Renderer {
            config,
            surface,
            device,
            queue,
            size_bind_group,
            size_buffer,
            rect: rect_renderer,
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.reconfigure();

            self.queue.write_buffer(
                &self.size_buffer,
                0,
                bytemuck::cast_slice(&[new_size.width as f32, new_size.height as f32]),
            );
        }
    }

    pub fn reconfigure(&mut self) {
        info!("Reconfiguring surface!");
        self.surface.configure(&self.device, &self.config);
    }

    pub fn prepare_render(&mut self) {
        self.rect.prepare_render();
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.rect.finish_render(&self.queue, &self.device);

        let surface_texture = self.surface.get_current_texture()?;
        let surface_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor {
                label: Some("Surface View"),
                ..Default::default()
            });
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Command Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &surface_view,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::RED),
                        store: true,
                    },
                    resolve_target: None,
                }],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.rect.pipeline);
            render_pass.set_bind_group(0, &self.size_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.rect.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.rect.instance_buffer.slice(..));
            render_pass
                .set_index_buffer(self.rect.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..6, 0, 0..(self.rect.instances.len() as u32));
        }

        self.queue.submit(Some(encoder.finish()));
        surface_texture.present();
        Ok(())
    }
}
