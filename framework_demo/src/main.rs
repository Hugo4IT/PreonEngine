use preon_engine::{
    components::PreonButton,
    events::{PreonEvent, WindowEventArgs},
    PreonComponent, PreonEngine,
};
use preon_framework::{run, PreonFramework};

struct App {}

impl PreonFramework for App {
    fn new(engine: &mut PreonEngine) -> Self {
        engine.root.add_child(PreonButton::new());

        Self {}
    }

    fn event(&mut self, event: &PreonEvent) {
        match event {
            PreonEvent::Window(WindowEventArgs::Closed) => {
                println!("Sadness");
            }
            _ => {}
        }
    }
}

fn main() {
    run::<App>();
}
