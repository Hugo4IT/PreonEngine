use std::any::{Any, TypeId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ComponentId(pub usize, pub TypeId);

#[derive(Debug)]
pub struct Component {
    pub id: ComponentId,
    pub data: Box::<dyn Any>,
}