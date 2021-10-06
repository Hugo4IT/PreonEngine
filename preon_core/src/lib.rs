use utils::{PreonLayout};

pub mod utils;

pub struct PreonCore {
    pub components: Vec<Box<dyn PreonComponent>>
}

impl PreonCore {
    pub fn init() -> Self {
        Self {
            components: Vec::new()
        }
    }

    pub fn update(&mut self) {
        
    }
}

pub trait PreonRenderer {
    fn start(&mut self, core: &PreonCore);
    fn update(&mut self, core: &mut PreonCore) -> bool;
    fn render(&mut self, core: &PreonCore);
}

pub trait PreonComponent {
    fn add_child(&mut self, new_child: dyn PreonComponent);
    fn layout(&mut self) -> PreonLayout;
}