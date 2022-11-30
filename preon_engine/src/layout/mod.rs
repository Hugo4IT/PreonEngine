use crate::components::PreonComponent;

pub mod rows;
pub mod columns;
pub mod container;

pub(crate) trait PreonLayoutProvider {
    fn layout(component: &mut PreonComponent);
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub enum PreonLayout {
    Rows,
    Columns,
    Container,
}

impl Default for PreonLayout {
    fn default() -> Self {
        Self::Rows
    }
}