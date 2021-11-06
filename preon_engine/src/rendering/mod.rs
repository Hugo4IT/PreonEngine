use crate::{events::PreonEventEmitter, types::{PreonColor, PreonVector}};

/// Describe how to render your UI component by pushing some PreonShapes to the PreonRenderPass
#[derive(Debug, Copy, Clone)]
pub enum PreonShape {
    Rect {
        position: PreonVector<i32>,
        size: PreonVector<i32>,
        color: PreonColor,
    },
}

/// Syntax sugar
pub type PreonRenderPass = PreonEventEmitter<PreonShape>;
