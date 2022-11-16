use alloc::{string::String, vec::Vec};

use crate::{
    events::PreonEventEmitter,
    types::{PreonColor, PreonVector, PreonCorners}, style::PreonTextStyle,
};

/// Describe how to render your UI component by pushing some PreonShapes to the PreonRenderPass
#[derive(Debug, Clone)]
pub enum PreonShape {
    Rect {
        position: PreonVector<i32>,
        size: PreonVector<i32>,
        color: PreonColor,
        index: Option<usize>,
        radius: PreonCorners,
    },
    Text {
        text_style: PreonTextStyle,
        color: PreonColor,
        position: PreonVector<i32>,
        size: PreonVector<i32>,
        text: String,
    },
}

pub struct PreonFont {
    index: usize,
}

impl PreonFont {
    pub(crate) fn new(index: usize) -> PreonFont {
        PreonFont { index }
    }

    #[inline]
    pub fn index(&self) -> usize {
        self.index
    }
}

pub trait IntoFont {
    fn get_font(self) -> Vec<u8>;
}

impl IntoFont for &[u8] {
    fn get_font(self) -> Vec<u8> {
        self.to_vec()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PreonImage {
    index: usize,
}

impl PreonImage {
    pub(crate) fn new(index: usize) -> PreonImage {
        PreonImage { index }
    }

    #[inline]
    pub fn index(&self) -> usize {
        self.index
    }
}

pub trait IntoImage {
    fn get_image(self) -> Vec<u8>;
}

impl IntoImage for &[u8] {
    fn get_image(self) -> Vec<u8> {
        self.to_vec()
    }
}

/// Used to load data into renderer
pub struct PreonRenderData {
    pub textures: Vec<Vec<u8>>,
    pub fonts: Vec<Vec<u8>>,
}

impl PreonRenderData {
    pub fn new() -> Self {
        Self {
            textures: Vec::new(),
            fonts: Vec::new(),
        }
    }
}

/// Syntax sugar
pub type PreonRenderPass = PreonEventEmitter<PreonShape>;
