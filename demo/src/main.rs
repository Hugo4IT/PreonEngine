#![no_std]
extern crate alloc;
use alloc::boxed::Box;

use preon_engine::{
    components::{panel::Panel, PANEL},
    types::color::Color,
    PreonEngine,
};

fn main() {
    env_logger::init();

    let mut engine = PreonEngine::new();

    let first = engine.add_component(None, PANEL, Box::new(Panel(Color(1.0, 0.0, 0.0, 0.0))));
    let second = engine.add_component(None, PANEL, Box::new(Panel(Color(0.0, 1.0, 0.0, 0.0))));
    let third = engine.add_component(None, PANEL, Box::new(Panel(Color(0.0, 0.0, 1.0, 0.0))));

    engine.start();
}
