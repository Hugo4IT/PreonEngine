use preon_engine::engine::{PreonComponent, PreonEngine, PreonRenderer};
use preon_module_opengl::PreonRendererOpenGL;

pub struct Preon {
    renderer: PreonRendererOpenGL,
    core: PreonEngine,
}

impl Preon {
    pub fn new() -> Self {
        Self {
            renderer: PreonRendererOpenGL::init(),
            core: PreonEngine::init(),
        }
    }

    pub fn start(&mut self) {
        self.renderer.start(&self.core);

        while !self.renderer.update(&mut self.core) {
            self.renderer.render(&self.core);
        }
    }

    pub fn add_child(&mut self, new_child: Box<dyn PreonComponent>) {
        self.core.root.add_child(new_child);
    }
}
