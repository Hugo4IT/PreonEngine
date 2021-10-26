use crate::types::{PreonBox, PreonColor};

#[derive(Debug, Clone)]
pub struct PreonComponent<T: PreonCustomComponentStack> {
    pub children: Option<Vec<PreonComponent<T>>>,
    pub model: PreonBox,
    pub data: PreonComponentStack<T>,
}

impl<T: PreonCustomComponentStack> PreonComponent<T> {
    pub fn new(component: PreonComponentStack<T>) -> PreonComponent<T> {
        PreonComponent {
            children: Some(Vec::new()),
            model: PreonBox::initial(),
            data: component,
        }
    }
}

pub trait PreonCustomComponentStack {}

#[derive(Debug, Clone)]
pub enum PreonComponentStack<T: PreonCustomComponentStack> {
    Custom(T),
    RectComponent { color: PreonColor },
    VBoxComponent
}