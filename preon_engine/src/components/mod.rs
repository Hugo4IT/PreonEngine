use crate::types::{PreonBox, PreonColor};

pub struct PreonComponent<T: PreonComponentStack> {
    children: Vec<T>,
    model: PreonBox,
    in_stack: T,
}

impl<T: PreonComponentStack> PreonComponent<T> {
    pub fn new(component: T) -> PreonComponent<T> {
        PreonComponent {
            children: Vec::new(),
            model: PreonBox::initial(),
            in_stack: component,
        }
    }
}

pub trait PreonComponentStack {
    fn get_default(c: PreonDefaultComponents) -> Self;
}

pub enum PreonDefaultComponents {
    RectComponent { color: PreonColor },
    VBoxComponent
}

impl PreonComponentStack for PreonDefaultComponents {
    fn get_default(c: PreonDefaultComponents) -> Self { c }
}