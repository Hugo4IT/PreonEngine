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
