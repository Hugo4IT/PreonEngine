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
        text_settings: u64,
        position: PreonVector<i32>,
        size: PreonVector<i32>,
        text: String,
    },
}

#[macro_export]
macro_rules! preon_font {
    ($path:expr $(,)?, $ext:expr $(,)?) => {
        &preon_engine::rendering::PreonFont {
            w100: Some(include_bytes!(concat!($path, "-Thin.", $ext))),
            w100i: Some(include_bytes!(concat!($path, "-ThinItalic.", $ext))),
            w200: Some(include_bytes!(concat!($path, "-ExtraLight.", $ext))),
            w200i: Some(include_bytes!(concat!($path, "-ExtraLightItalic.", $ext))),
            w300: Some(include_bytes!(concat!($path, "-Light.", $ext))),
            w300i: Some(include_bytes!(concat!($path, "-LightItalic.", $ext))),
            w400: Some(include_bytes!(concat!($path, "-Regular.", $ext))),
            w400i: Some(include_bytes!(concat!($path, "-Italic.", $ext))),
            w500: Some(include_bytes!(concat!($path, "-Medium.", $ext))),
            w500i: Some(include_bytes!(concat!($path, "-MediumItalic.", $ext))),
            w600: Some(include_bytes!(concat!($path, "-SemiBold.", $ext))),
            w600i: Some(include_bytes!(concat!($path, "-SemiBoldItalic.", $ext))),
            w700: Some(include_bytes!(concat!($path, "-Bold.", $ext))),
            w700i: Some(include_bytes!(concat!($path, "-BoldItalic.", $ext))),
            w800: Some(include_bytes!(concat!($path, "-ExtraBold.", $ext))),
            w800i: Some(include_bytes!(concat!($path, "-ExtraBoldItalic.", $ext))),
            w900: Some(include_bytes!(concat!($path, "-Black.", $ext))),
            w900i: Some(include_bytes!(concat!($path, "-BlackItalic.", $ext))),
        }
    };
}

/// Define a font with multiple weights, normal and italic versions
pub struct PreonFont {
    /// Weight 100 | Thin
    pub w100: Option<&'static [u8]>,
    /// Weight 100 italic | ThinItalic
    pub w100i: Option<&'static [u8]>,
    /// Weight 200 | ExtraLight
    pub w200: Option<&'static [u8]>,
    /// Weight 200 italic | ExtraLightItalic
    pub w200i: Option<&'static [u8]>,
    /// Weight 300 | Light
    pub w300: Option<&'static [u8]>,
    /// Weight 300 italic | LightItalic
    pub w300i: Option<&'static [u8]>,
    /// Weight 400 | Regular
    pub w400: Option<&'static [u8]>,
    /// Weight 400 italic | Italic
    pub w400i: Option<&'static [u8]>,
    /// Weight 500 | Medium
    pub w500: Option<&'static [u8]>,
    /// Weight 500 italic | MediumItalic
    pub w500i: Option<&'static [u8]>,
    /// Weight 600 | SemiBold
    pub w600: Option<&'static [u8]>,
    /// Weight 600 italic | SemiBoldItalic
    pub w600i: Option<&'static [u8]>,
    /// Weight 700 | Bold
    pub w700: Option<&'static [u8]>,
    /// Weight 700 Italic | BoldItalic
    pub w700i: Option<&'static [u8]>,
    /// Weight 800 | ExtraBold
    pub w800: Option<&'static [u8]>,
    /// Weight 800 Italic | ExtraBoldItalic
    pub w800i: Option<&'static [u8]>,
    /// Weight 900 | Black
    pub w900: Option<&'static [u8]>,
    /// Weight 900 Italic | BlackItalic
    pub w900i: Option<&'static [u8]>,
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
    pub fonts: &'static [&'static PreonFont],
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
