use preon_engine::{
    events::{PreonEvent, WindowEventArgs},
    PreonEngine, PreonRenderer,
};
use preon_module_opengl::PreonRendererOpenGL;

pub struct Preon<T: PreonFramework> {
    renderer: PreonRendererOpenGL,
    engine: PreonEngine,
    framework: T,
}

pub trait PreonFramework {
    fn new(engine: &mut PreonEngine) -> Self;
    fn event(&mut self, event: &PreonEvent);
}

pub fn run<T: PreonFramework>() {
    let mut engine = PreonEngine::new();
    let renderer = PreonRendererOpenGL::new();
    let framework = T::new(&mut engine);

    let mut preon: Preon<T> = Preon {
        renderer,
        engine,
        framework,
    };

    preon.renderer.start();
    while preon.renderer.update(&mut preon.engine) {
        preon.engine.update();

        let framework = &mut preon.framework;
        preon.engine.events.pull(|event| {
            framework.event(event);
        });

        preon.renderer.render(&mut preon.engine);
    }

    preon
        .framework
        .event(&PreonEvent::Window(WindowEventArgs::Closed));
}
