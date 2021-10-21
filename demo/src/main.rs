use preon_engine::{PreonEngine, components::{PreonComponentStack, PreonDefaultComponents}};
use preon_module_wgpu::preon;

pub enum MyComponentStack {
    Defaults(PreonDefaultComponents)
}

impl PreonComponentStack for MyComponentStack {
    fn get_default(c: PreonDefaultComponents) -> Self {
        Self::Defaults(c)
    }
}

fn main() {
    let engine = PreonEngine::<MyComponentStack>::new();
    preon::run(engine);

    // let mut pass = PreonRenderPass::new();

    // renderer.start();
    // while renderer.update(&mut engine.events) {
    //     engine.update();

    //     pass.push(PreonShape::Rect {
    //         position: PreonVector::new(0, 0),
    //         size: PreonVector::new(0, 0),
    //         color: PreonColor::from_hex("#da0037")
    //     });
    //     pass.flip();

    //     renderer.render(&mut pass);
    // }
}