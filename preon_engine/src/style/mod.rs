use crate::{
    types::{PreonColor, PreonAlignment, PreonBorder, PreonVector, PreonCorners},
    size, layout::PreonLayout, components::PreonComponentBuilder, rendering::PreonImage, prelude::PreonFont
};

pub trait PreonClass {
    fn style(self, builder: &mut PreonComponentBuilder) -> &mut PreonComponentBuilder;
}

#[derive(Debug, Clone)]
pub enum PreonBackground {
    Image(PreonImage),
    Color(PreonColor),
    None,
}

#[derive(Debug, Clone)]
pub struct PreonTextStyle {
    pub size: f32,
    pub font: Option<PreonFont>,
    pub vertical_align: PreonAlignment,
    pub horizontal_align: PreonAlignment,
}

impl Default for PreonTextStyle {
    fn default() -> Self {
        Self {
            size: 16.0,
            font: None,
            vertical_align: PreonAlignment::Start,
            horizontal_align: PreonAlignment::Start,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PreonStyle {
    pub background: PreonBackground,
    pub foreground_color: PreonColor,
    pub align_items: PreonAlignment,
    pub cross_align_items: PreonAlignment,
    pub layout: PreonLayout,
    pub margin: PreonBorder,
    pub padding: PreonBorder,
    pub border: PreonBorder,
    pub corner_radius: PreonCorners,
    pub size_flags: u8,
    pub min_size: PreonVector<i32>,
    pub text_style: PreonTextStyle,
}

impl PreonStyle {
    pub fn initial() -> PreonStyle {
        PreonStyle {
            background: PreonBackground::None,
            foreground_color: PreonColor::BLACK,
            align_items: PreonAlignment::Start,
            cross_align_items: PreonAlignment::Start,
            layout: PreonLayout::default(),
            margin: PreonBorder::zero(),
            padding: PreonBorder::zero(),
            border: PreonBorder::zero(),
            corner_radius: PreonCorners::ZERO,
            size_flags: size::FIT,
            min_size: PreonVector::zero(),
            text_style: PreonTextStyle::default(),
        }
    }

    pub fn inherit_from(style: &PreonStyle) -> PreonStyle {
        PreonStyle {
            foreground_color: style.foreground_color,
            text_style: style.text_style.clone(),
            ..Default::default()
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
    fn background_image(&mut self, image: &PreonImage) -> &mut PreonComponentBuilder;
    fn background_color(&mut self, color: PreonColor) -> &mut PreonComponentBuilder;
    fn foreground_color(&mut self, color: PreonColor) -> &mut PreonComponentBuilder;
    fn align_items(&mut self, alignment: PreonAlignment) -> &mut PreonComponentBuilder;
    fn cross_align_items(&mut self, alignment: PreonAlignment) -> &mut PreonComponentBuilder;
    fn layout(&mut self, layout: PreonLayout) -> &mut PreonComponentBuilder;
    fn margin(&mut self, margin: PreonBorder) -> &mut PreonComponentBuilder;
    fn padding(&mut self, padding: PreonBorder) -> &mut PreonComponentBuilder;
    fn border(&mut self, border: PreonBorder) -> &mut PreonComponentBuilder;
    fn corner_radius(&mut self, corners: PreonCorners) -> &mut PreonComponentBuilder;
    fn min_size(&mut self, min_size: PreonVector<i32>) -> &mut PreonComponentBuilder;
    fn fit_children(&mut self) -> &mut PreonComponentBuilder;
    fn fit_children_horizontally(&mut self) -> &mut PreonComponentBuilder;
    fn fit_children_vertically(&mut self) -> &mut PreonComponentBuilder;
    fn expand(&mut self) -> &mut PreonComponentBuilder;
    fn expand_horizontally(&mut self) -> &mut PreonComponentBuilder;
    fn expand_vertically(&mut self) -> &mut PreonComponentBuilder;
    fn style(&mut self, style: PreonStyle) -> &mut PreonComponentBuilder;
    fn apply(&mut self, class: impl PreonClass) -> &mut PreonComponentBuilder;
}

impl PreonComponentBuilderStyleExtension for PreonComponentBuilder {
    fn background_image(&mut self, image: &PreonImage) -> &mut PreonComponentBuilder {
        self.current_mut().style.background = PreonBackground::Image(image.clone());
        self
    }

    fn background_color(&mut self, color: PreonColor) -> &mut PreonComponentBuilder {
        self.current_mut().style.background = PreonBackground::Color(color);
        self
    }

    fn foreground_color(&mut self, color: PreonColor) -> &mut PreonComponentBuilder {
        self.current_mut().style.foreground_color = color;
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

    fn corner_radius(&mut self, corners: PreonCorners) -> &mut PreonComponentBuilder {
        self.current_mut().style.corner_radius = corners;
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

    fn apply(&mut self, class: impl PreonClass) -> &mut PreonComponentBuilder {
        let component = class.style(&mut PreonComponentBuilder::from_component(self.stack.pop().unwrap())).build();
        self.stack.push(component);
        self
    }
}

pub trait PreonComponentBuilderTextStyleExtension {
    fn text_vertical_align(&mut self, alignment: PreonAlignment) -> &mut PreonComponentBuilder;
    fn text_horizontal_align(&mut self, alignment: PreonAlignment) -> &mut PreonComponentBuilder;
    fn font<'a>(&mut self, font: &'a PreonFont) -> &mut PreonComponentBuilder;
    fn font_size(&mut self, size: f32) -> &mut PreonComponentBuilder;
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

    fn font(&mut self, font: &PreonFont) -> &mut PreonComponentBuilder {
        self.current_mut().style.text_style.font = Some(font.clone());
        self
    }

    fn font_size(&mut self, size: f32) -> &mut PreonComponentBuilder {
        self.current_mut().style.text_style.size = size;
        self
    }
}