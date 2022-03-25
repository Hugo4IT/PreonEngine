use std::any::TypeId;
use super::component::Component;

pub type SysFunc = fn(Vec<&mut Component>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct SystemId(pub usize);

#[derive(Clone)]
pub struct System {
    pub id: SystemId,
    pub query: Vec<TypeId>,
    pub function: SysFunc,
}