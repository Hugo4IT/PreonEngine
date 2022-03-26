use std::any::TypeId;

#[derive(Debug, Clone)]
pub struct Query {
    pub requirements: Vec<TypeId>,
}