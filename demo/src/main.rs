use std::mem::{size_of, size_of_val};

use preon_engine::{
    components::{panel::Panel, PANEL},
    types::color::Color,
    ComponentDataHolder, ComponentReference, PreonEngine,
};

fn main() {
    env_logger::init();

    let mut engine = PreonEngine::new();

    let first = engine.add_component(None, PANEL, Box::new(Panel(Color(1.0, 0.0, 0.0, 0.0))));
    let second = engine.add_component(None, PANEL, Box::new(Panel(Color(0.0, 1.0, 0.0, 0.0))));
    let third = engine.add_component(None, PANEL, Box::new(Panel(Color(0.0, 0.0, 1.0, 0.0))));

    println!("Before insert:");
    println!(
        "{}: {}",
        *first.borrow(),
        engine.get_component(&first).get_data::<Panel>().0
    );
    println!(
        "{}: {}",
        *second.borrow(),
        engine.get_component(&second).get_data::<Panel>().0
    );
    println!(
        "{}: {}",
        *third.borrow(),
        engine.get_component(&third).get_data::<Panel>().0
    );

    let inserted =
        engine.insert_component(None, 1, PANEL, Box::new(Panel(Color(0.0, 0.0, 0.0, 1.0))));

    println!("After insert(1):");
    println!(
        "{}: {}",
        *first.borrow(),
        engine.get_component(&first).get_data::<Panel>().0
    );
    println!(
        "{}: {}",
        *inserted.borrow(),
        engine.get_component(&inserted).get_data::<Panel>().0
    );
    println!(
        "{}: {}",
        *second.borrow(),
        engine.get_component(&second).get_data::<Panel>().0
    );
    println!(
        "{}: {}",
        *third.borrow(),
        engine.get_component(&third).get_data::<Panel>().0
    );

    println!("Base size of holder: {}", size_of::<ComponentDataHolder>());
    println!(
        "Actual size of holder: {}",
        size_of_val(engine.get_component(&first))
    );
    println!(
        "Base size of reference: {}",
        size_of::<ComponentReference>()
    );
    println!("Actual size of reference: {}", size_of_val(&first));

    engine.start();
}
