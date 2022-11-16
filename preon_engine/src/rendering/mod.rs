use alloc::string::String;

use crate::{
    events::PreonEventEmitter,
    types::{PreonColor, PreonVector}, style::PreonTextStyle,
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
        text_style: PreonTextStyle,
        color: PreonColor,
        position: PreonVector<i32>,
        size: PreonVector<i32>,
        text: String,
    },
}

/// ### Actual definition
///
/// ```
/// macro_rules! preon_font {
///     ($path:expr, $ext:expr) => { /* Code */ }
/// }
/// ```
///
/// ### Readable definition
///
/// ```
/// preon_font!(path: &'const str, ext: &'const str);
/// ```
///
/// ---
///
/// Automatically includes all font files following this naming scheme:
///
/// field       | filename
/// :---------- | :------------------------------
/// **`w100`**  | `{path}-Thin.{ext}`
/// **`w100i`** | `{path}-ThinItalic.{ext}`
/// **`w200`**  | `{path}-ExtraLight.{ext}`
/// **`w200i`** | `{path}-ExtraLightItalic.{ext}`
/// **`w300`**  | `{path}-Light.{ext}`
/// **`w300i`** | `{path}-LightItalic.{ext}`
/// **`w400`**  | `{path}-Regular.{ext}`
/// **`w400i`** | `{path}-Italic.{ext}`
/// **`w500`**  | `{path}-Medium.{ext}`
/// **`w500i`** | `{path}-MediumItalic.{ext}`
/// **`w600`**  | `{path}-SemiBold.{ext}`
/// **`w600i`** | `{path}-SemiBoldItalic.{ext}`
/// **`w700`**  | `{path}-Bold.{ext}`
/// **`w700i`** | `{path}-BoldItalic.{ext}`
/// **`w800`**  | `{path}-ExtraBold.{ext}`
/// **`w800i`** | `{path}-ExtraBoldItalic.{ext}`
/// **`w900`**  | `{path}-Black.{ext}`
/// **`w900i`** | `{path}-BlackItalic.{ext}`
///
/// # Example
///
/// ```
/// let render_data = PreonStaticRenderData {
///     textures: &[],
///     fonts: &[
///         preon_font!("../../res/Montserrat", "otf")
///     ]
/// };
/// ```
///
/// [1]: https://docs.microsoft.com/en-us/typography/opentype/spec/os2#usweightclass
#[macro_export]
macro_rules! preon_font {
    ($path:expr, $ext:expr) => {
        &preon_engine::rendering::PreonFontData {
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

/// Define a font with multiple weights, normal and italic versions.
///
/// ## Field names
///
/// Fields are named like this: `w{weight}[i]`, `weight` being the thickness ([Like CSS][1]), and `i`
/// stating if the font it italic (_skewed_).
///
/// ## Avoiding boiler-plate
///
/// Check the [`preon_font`] macro to automatically fill in the slots.
///
/// [1]: https://developer.mozilla.org/en-US/docs/Web/CSS/font-weight
pub struct PreonFontData {
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

impl Default for PreonFontData {
    fn default() -> Self {
        Self {
            w100: None,
            w100i: None,
            w200: None,
            w200i: None,
            w300: None,
            w300i: None,
            w400: None,
            w400i: None,
            w500: None,
            w500i: None,
            w600: None,
            w600i: None,
            w700: None,
            w700i: None,
            w800: None,
            w800i: None,
            w900: None,
            w900i: None,
        }
    }
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
    pub fonts: &'static [&'static PreonFontData],
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
