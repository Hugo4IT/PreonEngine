use preon_core::{PreonComponent, PreonCore, PreonRenderer};
use preon_renderer_opengl::PreonRendererOpenGL;

pub struct Preon {
    renderer: PreonRendererOpenGL,
    core: PreonCore
}

impl Preon {
    pub fn new() -> Self {
        Self {
            renderer: PreonRendererOpenGL::init(),
            core: PreonCore::init()
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