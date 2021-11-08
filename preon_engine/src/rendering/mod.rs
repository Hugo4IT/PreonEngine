use crate::{events::PreonEventEmitter, theme::PreonFont, types::{PreonColor, PreonVector}};

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
    }
}

pub struct PreonStaticRenderData {
    pub textures: &'static [&'static [u8]],
    pub strings: &'static [&'static str],
    pub fonts: &'static [&'static PreonFont],
}

/// Syntax sugar
pub type PreonRenderPass = PreonEventEmitter<PreonShape>;
