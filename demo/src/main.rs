use preon_engine::{PreonEngine, PreonRenderer, components::{PreonComponentStack, PreonDefaultComponents}, rendering::{PreonRenderPass, PreonShape}, types::{PreonColor, PreonVector}};
use preon_module_opengl::PreonRendererOpenGL;

pub enum MyComponentStack {
    Defaults(PreonDefaultComponents)
}

impl PreonComponentStack for MyComponentStack {
    fn get_default(c: PreonDefaultComponents) -> Self {
        Self::Defaults(c)
    }
}

fn main() {
    let mut engine: PreonEngine<MyComponentStack> = PreonEngine::new();
    let mut renderer = PreonRendererOpenGL::new();
    let mut pass = PreonRenderPass::new();

    renderer.start();
    while renderer.update(&mut engine.events) {
        engine.update();

        pass.push(PreonShape::Rect {
            position: PreonVector::new(0, 0),
            size: PreonVector::new(0, 0),
            color: PreonColor::from_hex("#da0037")
        });
        pass.flip();

        renderer.render(&mut pass);
    }
}