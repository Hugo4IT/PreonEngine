use preon_engine::engine::{PreonComponent, PreonEngine, PreonRenderer};
use preon_module_opengl::PreonRendererOpenGL;

pub struct Preon {
    pub renderer: PreonRendererOpenGL,
    pub engine: PreonEngine,
}

impl Preon {
    pub fn new() -> Self {
        let mut renderer = PreonRendererOpenGL::new();
        let engine = PreonEngine::new();

        Self {
            renderer,
            engine
        }
    }

    pub fn start(&mut self) {
        self.renderer.start();

        while self.renderer.update() {
            self.engine.update();
            self.renderer.render();
        }
    }

    pub fn add_child(&mut self, new_child: Box<dyn PreonComponent>) {
        self.engine.root.add_child(new_child);
    }
}
