use log::info;
use winit::{dpi::LogicalSize, window::Window};

pub struct Renderer {
    config: wgpu::SurfaceConfiguration,
    surface: wgpu::Surface,

    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl Renderer {
    pub fn new(window: &Window) -> Renderer {
        let size = window.inner_size().to_logical(window.scale_factor());
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
            format: surface.get_preferred_format(&adapter).unwrap_or(wgpu::TextureFormat::Rgba8Unorm),
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            height: size.height,
            width: size.width,
        };
        surface.configure(&device, &config);

        Renderer { config, surface, device, queue }
    }

    pub fn resize(&mut self, new_size: LogicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.reconfigure();
        }
    }

    pub fn reconfigure(&mut self) {
        info!("Reconfiguring surface!");
        self.surface.configure(&self.device, &self.config);
    }

    pub fn render(&self) -> Result<(), wgpu::SurfaceError> {
        let surface_texture = self.surface.get_current_texture()?;
        let surface_view = surface_texture.texture.create_view(&wgpu::TextureViewDescriptor {
            label: Some("Surface View"),
            ..Default::default()
        });
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Command Encoder")
        });

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[
                    wgpu::RenderPassColorAttachment {
                        view: &surface_view,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::RED),
                            store: true,
                        },
                        resolve_target: None
                    },
                ],
                depth_stencil_attachment: None,
            });
        }

        self.queue.submit(Some(encoder.finish()));
        surface_texture.present();
        Ok(())
    }
}
