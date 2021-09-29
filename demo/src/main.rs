use preon_core::PreonCore;
use preon_renderer_wgpu::PreonRendererWGPU;
use winit::{event::{Event, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder};

struct App {
    surface: Surface,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    size: PhysicalSize<u32>,
    core: PreonCore
}

impl App {
    pub fn new(core: PreonCore) -> Self {
        let load = async {
            let size = window.inner_size();
            let instance = Instance::new(Backends::all());
            let surface = unsafe { instance.create_surface(window) };
            let adapter = instance.request_adapter(
                &RequestAdapterOptions {
                    power_preference: PowerPreference::LowPower,
                    compatible_surface: Some(&surface),
                },
            ).await.unwrap();
    
            let (device, queue) = adapter.request_device(
                &DeviceDescriptor {
                    features: Features::empty(),
                    limits: Limits::default(),
                    label: None,
                }, None,
            ).await.unwrap();
    
            let config = SurfaceConfiguration {
                usage: TextureUsages::RENDER_ATTACHMENT,
                format: surface.get_preferred_format(&adapter).unwrap(),
                width: size.width,
                height: size.height,
                present_mode: wgpu::PresentMode::Fifo
            };
    
            surface.configure(&device, &config);
    
            (surface, device, queue, config, size)
        };
    
        let (surface, device, queue, config, size) = pollster::block_on(load);

        Self {surface, device, queue, config, size, core}
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }
}

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let core = PreonCore::init();
    let app = App::new(core);
    let mut renderer = PreonRendererWGPU::init(core);

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            _ => {}
        },
        _ => {}
    });
}