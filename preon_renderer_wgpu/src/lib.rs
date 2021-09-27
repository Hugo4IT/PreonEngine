use preon_core::{PreonCore, PreonRenderer};

pub struct PreonRendererWGPU {
    
}

impl PreonRenderer for PreonRendererWGPU {
    fn init() -> Self {
        
        PreonRendererWGPU {}
    }

    fn start(&mut self, core: &PreonCore) {
        
    }

    fn update(&mut self, core: &PreonCore) -> bool {
        

        true
    }

    fn render(&mut self, core: &mut PreonCore) {
        
    }

    fn register(&mut self, core: &mut PreonCore) {
        
    }
}
