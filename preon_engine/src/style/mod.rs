use crate::{
    types::{PreonColor, PreonAlignment, PreonBorder, PreonVector},
    size, layout::PreonLayout, components::PreonComponentBuilder
};

use self::image::PreonImage;

pub mod image;

#[derive(Debug, Clone, Copy)]
pub enum PreonBackground {
    Image(PreonImage),
    Color(PreonColor),
    None,
}

#[derive(Debug, Clone, Copy)]
pub enum PreonForeground {
    Color(PreonColor),
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
    pub foreground: PreonForeground,
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
            foreground: PreonForeground::Color(PreonColor::BLACK),
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

    pub fn has_size_flag(&self, flag: u8) -> bool {
        (self.size_flags & flag) == flag
    }
}

impl Default for PreonStyle {
    fn default() -> Self {
        Self::initial()
    }
}

pub trait PreonComponentBuilderStyleExtension {
    fn background_image(&mut self, image: PreonImage) -> &mut PreonComponentBuilder;
    fn background_color(&mut self, color: PreonColor) -> &mut PreonComponentBuilder;
    fn foreground_color(&mut self, color: PreonColor) -> &mut PreonComponentBuilder;
    fn align_items(&mut self, alignment: PreonAlignment) -> &mut PreonComponentBuilder;
    fn cross_align_items(&mut self, alignment: PreonAlignment) -> &mut PreonComponentBuilder;
    fn layout(&mut self, layout: PreonLayout) -> &mut PreonComponentBuilder;
    fn margin(&mut self, margin: PreonBorder) -> &mut PreonComponentBuilder;
    fn padding(&mut self, padding: PreonBorder) -> &mut PreonComponentBuilder;
    fn border(&mut self, border: PreonBorder) -> &mut PreonComponentBuilder;
    fn min_size(&mut self, min_size: PreonVector<i32>) -> &mut PreonComponentBuilder;
    fn fit_children(&mut self) -> &mut PreonComponentBuilder;
    fn fit_children_horizontally(&mut self) -> &mut PreonComponentBuilder;
    fn fit_children_vertically(&mut self) -> &mut PreonComponentBuilder;
    fn expand(&mut self) -> &mut PreonComponentBuilder;
    fn expand_horizontally(&mut self) -> &mut PreonComponentBuilder;
    fn expand_vertically(&mut self) -> &mut PreonComponentBuilder;
    fn style(&mut self, style: PreonStyle) -> &mut PreonComponentBuilder;
}

impl PreonComponentBuilderStyleExtension for PreonComponentBuilder {
    fn background_image(&mut self, image: PreonImage) -> &mut PreonComponentBuilder {
        self.current_mut().style.background = PreonBackground::Image(image);
        self
    }

    fn background_color(&mut self, color: PreonColor) -> &mut PreonComponentBuilder {
        self.current_mut().style.background = PreonBackground::Color(color);
        self
    }

    fn foreground_color(&mut self, color: PreonColor) -> &mut PreonComponentBuilder {
        self.current_mut().style.foreground = PreonForeground::Color(color);
        self
    }

    fn align_items(&mut self, alignment: PreonAlignment) -> &mut PreonComponentBuilder {
        self.current_mut().style.align_items = alignment;
        self
    }

    fn cross_align_items(&mut self, alignment: PreonAlignment) -> &mut PreonComponentBuilder {
        self.current_mut().style.cross_align_items = alignment;
        self
    }

    fn layout(&mut self, layout: PreonLayout) -> &mut PreonComponentBuilder {
        self.current_mut().style.layout = layout;
        self
    }

    fn margin(&mut self, margin: PreonBorder) -> &mut PreonComponentBuilder {
        self.current_mut().style.margin = margin;
        self
    }

    fn padding(&mut self, padding: PreonBorder) -> &mut PreonComponentBuilder {
        self.current_mut().style.padding = padding;
        self
    }

    fn border(&mut self, border: PreonBorder) -> &mut PreonComponentBuilder {
        self.current_mut().style.border = border;
        self
    }

    fn min_size(&mut self, min_size: PreonVector<i32>) -> &mut PreonComponentBuilder {
        self.current_mut().style.min_size = min_size;
        self
    }

    fn fit_children(&mut self) -> &mut PreonComponentBuilder {
        self.current_mut().style.size_flags |= size::FIT;
        self
    }

    fn fit_children_horizontally(&mut self) -> &mut PreonComponentBuilder {
        self.current_mut().style.size_flags |= size::horizontal::FIT;
        self
    }

    fn fit_children_vertically(&mut self) -> &mut PreonComponentBuilder {
        self.current_mut().style.size_flags |= size::vertical::FIT;
        self
    }

    fn expand(&mut self) -> &mut PreonComponentBuilder {
        self.current_mut().style.size_flags |= size::EXPAND;
        self
    }

    fn expand_horizontally(&mut self) -> &mut PreonComponentBuilder {
        self.current_mut().style.size_flags |= size::horizontal::EXPAND;
        self
    }

    fn expand_vertically(&mut self) -> &mut PreonComponentBuilder {
        self.current_mut().style.size_flags |= size::vertical::EXPAND;
        self
    }

    fn style(&mut self, style: PreonStyle) -> &mut PreonComponentBuilder {
        self.current_mut().style = style;
        self
    }
}

pub trait PreonComponentBuilderTextStyleExtension {
    fn text_vertical_align(&mut self, alignment: PreonAlignment) -> &mut PreonComponentBuilder;
    fn text_horizontal_align(&mut self, alignment: PreonAlignment) -> &mut PreonComponentBuilder;
    fn bold(&mut self) -> &mut PreonComponentBuilder;
    fn italic(&mut self) -> &mut PreonComponentBuilder;
}

impl PreonComponentBuilderTextStyleExtension for PreonComponentBuilder {
    fn text_vertical_align(&mut self, alignment: PreonAlignment) -> &mut PreonComponentBuilder {
        self.current_mut().style.text_style.vertical_align = alignment;
        self
    }

    fn text_horizontal_align(&mut self, alignment: PreonAlignment) -> &mut PreonComponentBuilder {
        self.current_mut().style.text_style.horizontal_align = alignment;
        self
    }

    fn bold(&mut self) -> &mut PreonComponentBuilder {
        self.current_mut().style.text_style.bold = true;
        self
    }

    fn italic(&mut self) -> &mut PreonComponentBuilder {
        self.current_mut().style.text_style.italic = true;
        self
    }
}