use preon_engine::{app::App, context::PreonContext, defaults::*};

#[derive(Debug, Clone)]
pub struct MyAppState {
    todos: Vec<(bool, String)>,
}

fn main() {
    env_logger::init();

    let mut app = App::new();
    app.add_window("ToDo Application", ui);
    app.set_state(MyAppState {
        todos: vec![
            (true, String::from("Item 1")),
            (false, String::from("Item 2")),
            (false, String::from("Item 3")),
        ],
    });
    app.start();
}

fn ui(ctx: &mut PreonContext, state: &mut MyAppState) {
    for (done, _label) in state.todos.iter_mut() {
        ctx.begin_horizontal();
        ctx.checkbox(done);
        ctx.checkbox(done);
        ctx.end_horizontal();
    }

    println!("{:?}", state);
}
