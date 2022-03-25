use preon_ecs::{system, ECS, fn_with_args};

struct Printer(pub String);

system!(printer, 1);
fn printer(printer: &mut Printer) {
    println!("{}", printer.0);
}

fn main() {
    let mut ecs = ECS::new();
    ecs.add_system(printer as fn_with_args!(1), printer::system);
    ecs.add_entity((
        Printer(String::from("Hello, System!")),
    ));

    for _i in 0..5 {
        ecs.update();
    }
}
