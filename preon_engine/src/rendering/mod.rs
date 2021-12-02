use crate::{
    events::PreonEventEmitter,
    types::{PreonColor, PreonVector},
};

/// Describe how to render your UI component by pushing some PreonShapes to the PreonRenderPass
#[derive(Debug, Clone)]
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
    Text {
        font_index: usize,
        position: PreonVector<i32>,
        size: PreonVector<i32>,
        text: String,
    },
}

/// Data that gets bundled inside the executable at compile-time, this removes the need
/// for bundling extra files with your application, making the executable the only thing
/// the user needs for your application (no installers!)
///
/// ## Example
///
/// ```
/// let static_render_data = PreonStaticRenderData {
///     textures: &[
///         include_bytes!("../../../res/mm2wood.png"),
///         include_bytes!("../../../res/juan.png"),
///     ],
///     fonts: &[
///         include_bytes!("../../../res/Montserrat-Regular.ttf")
///     ]
/// };
/// ```
pub struct PreonStaticRenderData {
    pub textures: &'static [&'static [u8]],
    pub fonts: &'static [&'static [u8]],
}

impl PreonStaticRenderData {
    pub fn empty() -> Self {
        Self {
            textures: &[],
            fonts: &[],
        }
    }
}

/// Syntax sugar
pub type PreonRenderPass = PreonEventEmitter<PreonShape>;
