use preon::prelude::*;

#[derive(Component)]
pub struct App {
    #[bind] count: usize,
    #[bind] s: String,

    // Generated by Component
    preon_binding_count: usize,
    preon_binding_s: String,
    preon_mutated: bool,
}

// #[derive(Component)] generates this
{
    impl Component for App {
        fn take_mutated(&mut self) -> bool {
            let mutated = self.preon_mutated;
            self.preon_mutated = false;

            mutated
        }
    }

    impl App {
        fn mut_count(&mut self, callback: impl FnOnce(&mut usize)) {
            callback(&mut self.preon_binding_count);

            self.preon_mutated = true;
        }

        fn set_count(&mut self, value: usize) {
            self.preon_binding_count = value;
            self.preon_mutated = true;
        }

        fn count(&self) -> &usize {
            &self.preon_binding_count
        }

        fn mut_s(&mut self, callback: impl FnOnce(&mut String)) {
            callback(&mut self.preon_binding_s);

            self.preon_mutated = true;
        }

        fn set_s(&mut self, value: String) {
            self.preon_binding_s = value;
            self.preon_mutated = true;
        }

        fn s(&self) -> &String {
            &self.preon_binding_s
        }
    }
}

impl App {
    fn counter_button_clicked(&mut self, event: &mut PreonEvent<ClickEvent>) {
        self.mut_count(|c| *c += 1);
        self.set_s(if self.count() == 1 {
            String::from("s")
        } else {
            String::new()
        });
    }
}

impl View for App {
    preon::view! {
        Text(txt: "Counter", margin_bottom: 8px)
        Button(on_click: counter_button_clicked) {
            Text(txt: "You pressed the button {count} time{s}.")
        }
    }
}

// Generated by preon::view!

{
    fn view(&self, engine: &mut PreonEngine, parent: PreonComponentHandle) {
        {
            let mut comp1 = Text::default();
            comp1.txt.push(TextPart::Text(String::from("Counter")));
            comp1.margin_bottom = Unit::Pixels(8);
            let mut comp1_handle = engine.push_component(parent, comp1);
        }
        {
            let mut comp2 = Button::default();
            comp2.on_click = |instance, event| {
                instance.counter_button_clicked(event)
            };
            let mut comp2_handle = engine.push_component(parent, comp2);
            
            {
                let mut comp3 = Text::default();
                comp3.txt.push(TextPart::Text(String::from("You pressed the button ")));
                comp3.txt.push(TextPart::Bind(|instance| instance.count()));
                comp3.txt.push(TextPart::Text(String::from(" time")));
                comp3.txt.push(TextPart::Bind(|instance| instance.s()));
                comp3.txt.push(TextPart::Text(String::from(".")));
                engine.push_child(comp2_handle, comp3);
            }
        }
    }
}

fn main() {
    preon::run<App>();
}
