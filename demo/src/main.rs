use preon::Preon;
use preon_renderer_wgpu::PreonRendererWGPU;

fn main() {
    let mut ui: Preon<PreonRendererWGPU> = preon::init::<PreonRendererWGPU>();
    preon::start(&mut ui);
}