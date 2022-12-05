use core::cell::RefCell;

use alloc::{string::String, vec::{Vec, Drain}, rc::Rc};

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

#[derive(Debug, Clone)]
pub struct PreonFont {
    index: Rc<RefCell<usize>>,
}

impl PreonFont {
    pub(crate) fn new(index: Rc<RefCell<usize>>) -> PreonFont {
        PreonFont { index }
    }

    #[inline]
    pub fn index(&self) -> usize {
        *self.index.borrow()
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

#[derive(Debug, Clone)]
#[repr(C)]
pub struct PreonImage {
    index: Rc<RefCell<usize>>,
}

impl PreonImage {
    pub(crate) fn new(index: Rc<RefCell<usize>>) -> PreonImage {
        PreonImage { index }
    }

    #[inline]
    pub fn index(&self) -> usize {
        *self.index.borrow()
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
pub struct PreonRendererLoadOperations {
    pub textures: Vec<Vec<u8>>,
    pub unload_textures: Vec<usize>,
    pub fonts: Vec<Vec<u8>>,
    pub unload_fonts: Vec<usize>,
}

impl PreonRendererLoadOperations {
    pub fn new() -> Self {
        Self {
            textures: Vec::new(),
            unload_textures: Vec::new(),
            fonts: Vec::new(),
            unload_fonts: Vec::new(),
        }
    }

    pub fn take_textures(&mut self) -> Drain<Vec<u8>> {
        self.textures.drain(..)
    }

    pub fn take_unload_textures(&mut self) -> Drain<usize> {
        self.unload_textures.drain(..)
    }

    pub fn take_fonts(&mut self) -> Drain<Vec<u8>> {
        self.fonts.drain(..)
    }

    pub fn take_unload_fonts(&mut self) -> Drain<usize> {
        self.unload_fonts.drain(..)
    }
}

/// Syntax sugar
pub type PreonRenderPass = PreonEventEmitter<PreonShape>;
