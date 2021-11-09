use crate::{
    events::PreonEventEmitter,
    types::{PreonColor, PreonVector},
};

/// Describe how to render your UI component by pushing some PreonShapes to the PreonRenderPass
#[derive(Debug, Copy, Clone)]
pub enum PreonShape {
    Rect {
        position: PreonVector<i32>,
        size: PreonVector<i32>,
        color: PreonColor,
    },
    StaticTexture {
        position: PreonVector<i32>,
        size: PreonVector<i32>,
        index: usize,
    },
    StaticText {
        position: PreonVector<i32>,
        size: PreonVector<i32>,
        index: usize,
    },
}

pub struct PreonStaticRenderData {
    pub textures: &'static [&'static [u8]],
}

/// Syntax sugar
pub type PreonRenderPass = PreonEventEmitter<PreonShape>;
