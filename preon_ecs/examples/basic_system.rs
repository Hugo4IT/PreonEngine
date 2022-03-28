use preon_ecs::{system, ECS, fn_with_args};

struct Printer(pub String);
fn printer(printer: &mut Printer) {
    println!("{}", printer.0);
}
system!(printer, 1);

struct Counter(pub usize);
fn counter(counter: &mut Counter) {
    counter.0 += 1;
}
system!(counter, 1);

struct CounterPrinter;
fn counter_printer(counter: &mut Counter, _: &mut CounterPrinter) {
    println!("{}", counter.0);
}
system!(counter_printer, 2);

fn main() {
    let mut ecs = ECS::new();
    ecs.add_system(printer as fn_with_args!(1), printer::system);
    ecs.add_system(counter as fn_with_args!(1), counter::system);
    ecs.add_system(counter_printer as fn_with_args!(2), counter_printer::system);
    ecs.add_entity((
        Printer(String::from("Hello, System!")),
    ));
    ecs.add_entity((
        Counter(0),
        CounterPrinter,
    ));
    ecs.add_entity((
        Counter(0),
        CounterPrinter,
    ));
    ecs.add_entity((
        CounterPrinter,
        Counter(0),
    ));

    for _i in 0..5 {
        ecs.update();
    }
}
