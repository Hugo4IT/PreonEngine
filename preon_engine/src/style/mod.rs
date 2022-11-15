use alloc::{format, string::String};

use crate::{types::{PreonColor, PreonAlignment, PreonBorder, PreonVector}, size, layout::PreonLayout};

use core::fmt::Display;

use self::image::PreonImage;

pub mod image;

#[derive(Debug, Clone, Copy)]
pub enum PreonBackground {
    Image(PreonImage),
    Color(PreonColor),
    None,
}

#[derive(Debug, Clone, Copy)]
pub struct PreonTextStyle {
    pub size: u16,
    pub font_index: u16,
    pub bold: bool,
    pub italic: bool,
    pub vertical_align: PreonAlignment,
    pub horizontal_align: PreonAlignment,
}

impl Default for PreonTextStyle {
    fn default() -> Self {
        Self {
            size: 16,
            font_index: 0,
            bold: false,
            italic: false,
            vertical_align: PreonAlignment::Start,
            horizontal_align: PreonAlignment::Start,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PreonStyle {
    pub background: PreonBackground,
    pub foreground_color: PreonColor,
    pub align_items: PreonAlignment,
    pub cross_align_items: PreonAlignment,
    pub layout: PreonLayout,
    pub margin: PreonBorder,
    pub padding: PreonBorder,
    pub border: PreonBorder,
    pub size_flags: u8,
    pub min_size: PreonVector<i32>,
    pub text_style: PreonTextStyle,
}

impl PreonStyle {
    pub fn initial() -> PreonStyle {
        PreonStyle {
            background: PreonBackground::None,
            foreground_color: PreonColor::WHITE,
            align_items: PreonAlignment::Start,
            cross_align_items: PreonAlignment::Start,
            layout: PreonLayout::default(),
            margin: PreonBorder::zero(),
            padding: PreonBorder::zero(),
            border: PreonBorder::zero(),
            size_flags: size::FIT,
            min_size: PreonVector::zero(),
            text_style: PreonTextStyle::default(),
        }
    }

    pub fn has_flag(&self, flag: u8) -> bool {
        (self.size_flags & flag) == flag
    }
}

impl Default for PreonStyle {
    fn default() -> Self {
        Self::initial()
    }
}

impl Display for PreonStyle {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Self {
            margin,
            padding,
            border,
            size_flags,
            min_size,
            ..
        } = Self::default();

        let mut result = String::new();

        fn push_result(dest: &mut String, res: &str) {
            if dest.is_empty() {
                dest.push_str(res)
            } else {
                dest.push_str(&format!(", {}", res))
            }
        }

        if self.margin != margin {
            push_result(&mut result, &format!("margin: {}", self.margin))
        }
        if self.padding != padding {
            push_result(&mut result, &format!("padding: {}", self.padding))
        }
        if self.border != border {
            push_result(&mut result, &format!("border: {}", self.border))
        }

        if self.size_flags != size_flags {
            let mut flags = String::new();

            fn push_flag(dest: &mut String, flg: &str) {
                if dest.is_empty() {
                    dest.push_str(flg);
                } else {
                    dest.push_str(&format!(" | {}", flg));
                }
            }

            if self.has_flag(size::FIT) {
                push_flag(&mut flags, "FIT");
            } else if self.has_flag(size::horizontal::FIT) {
                push_flag(&mut flags, "HORIZONTAL_FIT");
            } else if self.has_flag(size::vertical::FIT) {
                push_flag(&mut flags, "VERTICAL_FIT");
            }

            if self.has_flag(size::EXPAND) {
                push_flag(&mut flags, "EXPAND");
            } else if self.has_flag(size::horizontal::EXPAND) {
                push_flag(&mut flags, "HORIZONTAL_EXPAND");
            } else if self.has_flag(size::vertical::EXPAND) {
                push_flag(&mut flags, "VERTICAL_EXPAND");
            }

            push_result(&mut result, &format!("size_flags: {}", flags));
        }

        if self.min_size != min_size {
            push_result(&mut result, &format!("min_size: {}", self.min_size));
        }

        write!(f, "{}", result)
    }
}