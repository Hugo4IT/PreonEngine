use preon_core::{PreonCore, PreonRenderer};
use wgpu::{Backends, Device, DeviceDescriptor, Features, Instance, Limits, PowerPreference, Queue, RequestAdapterOptions, Surface, SurfaceConfiguration, TextureUsages};
use winit::{dpi::PhysicalSize, window::Window};

pub struct PreonRendererWGPU {
    surface: Surface,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    size: PhysicalSize<u32>,
    core: PreonCore
}

impl PreonRendererWGPU {
    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }
}

impl PreonRenderer for PreonRendererWGPU {
    fn init(core: PreonCore) -> Self {
        
    }

    fn start(&mut self) {
        
    }

    fn update(&mut self) {
        
    }

    fn render(&mut self) {
        
    }

    fn register(&mut self) {
        
    }
}
