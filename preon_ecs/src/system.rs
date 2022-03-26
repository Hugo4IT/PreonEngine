use crate::query::Query;

use super::component::Component;

pub type SysFunc = fn(&mut Vec<Option<Component>>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct SystemId(pub usize);

#[derive(Clone)]
pub struct System {
    pub id: SystemId,
    pub query: Query,
    pub function: SysFunc,
}