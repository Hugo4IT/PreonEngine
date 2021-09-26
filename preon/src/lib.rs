use preon_core::{PreonCore, PreonRenderer};
use preon_renderer_opengl::PreonRendererOpenGL;

pub struct Preon {
    core: PreonCore,
    renderer: PreonRendererOpenGL
}

impl Preon {
    pub fn init() -> Self {
        Self {
            core: PreonCore::init(),
            renderer: PreonRendererOpenGL::init()
        }
    }

    pub fn start(&mut self, start_loop: bool) {
        self.renderer.start(&self.core);
    
        if start_loop {
            while !self.update() {
                self.render();
            }
        }
    }

    pub fn update(&mut self) -> bool {
        self.core.update();
        self.renderer.update(&self.core)
    }

    #[inline(always)]
    pub fn render(&mut self) {
        self.renderer.render(&mut self.core);
    }
}
