use component::PreonComponent;

pub mod component;

#[cfg(feature="utils")]
pub mod utils;

pub struct PreonCore {
    components: Vec<PreonComponent>
}

impl PreonCore {
    pub fn init() -> Self {
        Self {
            components: Vec::new()
        }
    }
}

pub trait PreonRenderer {
    fn init(core: &PreonCore) -> Self;
    fn start(&mut self, core: &PreonCore);
    fn update(&mut self, core: &PreonCore);
    fn render(&mut self, core: &PreonCore);
}