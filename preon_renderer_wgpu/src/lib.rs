use preon_api_renderer::PreonRenderer;
use preon_core::PreonCore;

pub struct PreonRendererWGPU {

}

impl PreonRenderer for PreonRendererWGPU {
    fn init(core: &PreonCore) -> Self {
        PreonRendererWGPU {  }
    }

    fn start(self: &mut Self, core: &PreonCore) {
        
    }

    fn update(self: &mut Self, core: &PreonCore) {
        
    }

    fn render(self: &Self, core: &PreonCore) {
        
    }
}