use preon_ecs::{system, ECS, fn_with_args};

struct Printer(pub String);

system!(printer, 2);
fn printer(printer: &mut Printer, printer2: &mut Printer) { 
}

fn main() {
    let mut ecs = ECS::new();
    let _p_entity = ecs.add_entity((
        Printer(String::from("Hello, System!")),
    ));
    let _p_system = ecs.add_system(printer as fn_with_args!(2), printer::system);

    for _i in 0..5 {
        ecs.update();
    }
}
