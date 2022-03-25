use super::component::ComponentId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct EntityId(pub usize);

#[derive(Debug, Clone)]

pub struct Entity {
    pub id: EntityId,
    pub components: Vec<ComponentId>,
}