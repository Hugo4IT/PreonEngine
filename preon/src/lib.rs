use preon_core::{PreonCore, PreonRenderer};
use preon_renderer_wgpu::PreonRendererWGPU;

pub struct Preon {
    core: PreonCore,
    renderer: PreonRendererWGPU
}

pub fn init() -> Preon {
    let core = PreonCore::init();
    let renderer = PreonRendererWGPU::init(&core);

    Preon { core, renderer }
}

pub fn start(preon: &mut Preon) {
    preon.renderer.start(&preon.core);
}