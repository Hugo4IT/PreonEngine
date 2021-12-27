use preon_engine::{components::PANEL, types::Color, PreonEngine};

fn main() {
    env_logger::init();

    let mut engine = PreonEngine::new();

    engine.add_component(None, PANEL, Box::new(Color(1.0, 0.0, 0.0, 0.0)));
    engine.add_component(None, PANEL, Box::new(Color(0.0, 1.0, 0.0, 0.0)));
    engine.add_component(None, PANEL, Box::new(Color(0.0, 0.0, 1.0, 0.0)));

    engine.start();
}
