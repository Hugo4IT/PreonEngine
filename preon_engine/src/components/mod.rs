use alloc::{
    borrow::ToOwned,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use core::fmt::Debug;

use crate::{
    events::{PreonButtonState, PreonEvent},
    layout::{
        columns::PreonColumnsLayoutProvider, container::PreonContainerLayoutProvider,
        rows::PreonRowsLayoutProvider, PreonLayout,
    },
    rendering::{PreonRenderPass, PreonShape},
    style::{PreonBackground, PreonStyle},
    types::{PreonAlignment, PreonColor, PreonRect, PreonVector},
    PreonComponentHandle,
};

pub mod button;
pub mod hbox;
pub mod label;
pub mod panel;
pub mod static_texture;
pub mod vbox;

#[repr(transparent)]
#[derive(Clone)]
pub struct ExcludeFromDebug<T: Clone>(T);

impl<T: Clone> core::fmt::Debug for ExcludeFromDebug<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ExcludeFromDebug<T>()")
    }
}

/// A UI component
#[derive(Debug, Clone)]
pub struct PreonComponent {
    pub parent: Option<PreonComponentHandle>,
    pub children: Vec<PreonComponentHandle>,
    pub style: PreonStyle,
    pub text: String,
    pub inner_size: PreonVector<i32>,
    pub inner_position: PreonVector<i32>,
    pub mouse_events: bool,
}

impl PreonComponent {
    pub fn new() -> PreonComponent {
        PreonComponent {
            parent: None,
            children: Vec::new(),
            style: PreonStyle::default(),
            text: String::new(),
            inner_size: PreonVector::zero(),
            inner_position: PreonVector::zero(),
            mouse_events: false,
        }
    }

    #[inline(always)]
    pub fn set_content_position(&mut self, new_position: PreonVector<i32>) {
        self.inner_position = new_position - self.style.padding.top_left();
    }

    #[inline(always)]
    pub fn get_content_position(&self) -> PreonVector<i32> {
        self.inner_position + self.style.padding.top_left()
    }

    #[inline(always)]
    pub fn get_content_rect(&self) -> PreonRect<i32> {
        PreonRect::new(self.get_content_position(), self.get_content_size())
    }

    #[inline(always)]
    pub fn set_inner_position(&mut self, new_position: PreonVector<i32>) {
        self.inner_position = new_position;
    }

    #[inline(always)]
    pub fn get_inner_position(&self) -> PreonVector<i32> {
        self.inner_position
    }

    #[inline(always)]
    pub fn get_inner_rect(&self) -> PreonRect<i32> {
        PreonRect::new(self.get_inner_position(), self.get_inner_size())
    }

    #[inline(always)]
    pub fn set_border_position(&mut self, new_position: PreonVector<i32>) {
        self.inner_position = new_position + self.style.border.top_left();
    }

    #[inline(always)]
    pub fn get_border_position(&self) -> PreonVector<i32> {
        self.inner_position - self.style.border.top_left()
    }

    #[inline(always)]
    pub fn get_border_rect(&self) -> PreonRect<i32> {
        PreonRect::new(self.get_border_position(), self.get_border_size())
    }

    #[inline(always)]
    pub fn set_outer_position(&mut self, new_position: PreonVector<i32>) {
        self.inner_position =
            new_position + self.style.border.top_left() + self.style.margin.top_left();
    }

    #[inline(always)]
    pub fn get_outer_position(&self) -> PreonVector<i32> {
        self.inner_position - self.style.border.top_left() - self.style.margin.top_left()
    }

    #[inline(always)]
    pub fn get_outer_rect(&self) -> PreonRect<i32> {
        PreonRect::new(self.get_outer_position(), self.get_outer_size())
    }

    #[inline(always)]
    pub fn set_content_size(&mut self, new_size: PreonVector<i32>) {
        self.inner_size = new_size + self.style.padding;
    }

    #[inline(always)]
    pub fn get_content_size(&self) -> PreonVector<i32> {
        self.get_inner_size() - self.style.padding
    }

    #[inline(always)]
    pub fn set_inner_size(&mut self, new_size: PreonVector<i32>) {
        self.inner_size = new_size;
    }

    #[inline(always)]
    pub fn get_inner_size(&self) -> PreonVector<i32> {
        PreonVector::new(
            self.inner_size.x.max(self.style.min_size.x),
            self.inner_size.y.max(self.style.min_size.y),
        )
    }

    #[inline(always)]
    pub fn set_border_size(&mut self, new_size: PreonVector<i32>) {
        self.inner_size = new_size - self.style.border
    }

    #[inline(always)]
    pub fn get_border_size(&self) -> PreonVector<i32> {
        self.get_inner_size() + self.style.border
    }

    #[inline(always)]
    pub fn set_outer_size(&mut self, new_size: PreonVector<i32>) {
        self.inner_size = new_size - self.style.margin - self.style.border;
    }

    #[inline(always)]
    pub fn get_outer_size(&self) -> PreonVector<i32> {
        self.get_inner_size() + self.style.border + self.style.margin
    }

    #[inline(always)]
    pub fn set_content_size_x(&mut self, new_x: i32) {
        self.set_inner_size_x(new_x + self.style.padding.x());
    }

    #[inline(always)]
    pub fn set_inner_size_x(&mut self, new_x: i32) {
        self.inner_size.x = new_x;
    }

    #[inline(always)]
    pub fn set_border_size_x(&mut self, new_x: i32) {
        self.set_inner_size_x(new_x - self.style.margin.x());
    }

    #[inline(always)]
    pub fn set_outer_size_x(&mut self, new_x: i32) {
        self.set_inner_size_x(new_x - self.style.margin.x() - self.style.border.x());
    }

    #[inline(always)]
    pub fn set_content_size_y(&mut self, new_y: i32) {
        self.set_inner_size_y(new_y + self.style.padding.y());
    }

    #[inline(always)]
    pub fn set_inner_size_y(&mut self, new_y: i32) {
        self.inner_size.y = new_y;
    }

    #[inline(always)]
    pub fn set_border_size_y(&mut self, new_y: i32) {
        self.set_inner_size_y(new_y - self.style.margin.y());
    }

    #[inline(always)]
    pub fn set_outer_size_y(&mut self, new_y: i32) {
        self.set_inner_size_y(new_y - self.style.margin.y() - self.style.border.y());
    }

    pub(crate) fn layout(&mut self) {
        use crate::layout::PreonLayoutProvider;

        for &child in self.children.iter() {
            child.layout();
        }

        match self.style.layout {
            PreonLayout::Rows => PreonRowsLayoutProvider::layout(self),
            PreonLayout::Columns => PreonColumnsLayoutProvider::layout(self),
            PreonLayout::Container => PreonContainerLayoutProvider::layout(self),
        }

        for &child in self.children.iter() {
            child.layout();
        }
    }

    pub(crate) fn render(&mut self, pass: &mut PreonRenderPass) {
        let stages = [
            PreonComponentRenderStage::Background {
                position: self.get_inner_position(),
                size: self.get_inner_size(),
            },
            PreonComponentRenderStage::Foreground {
                position: self.get_content_position(),
                size: self.get_content_size(),
            },
        ];

        for stage in stages {
            match stage {
                PreonComponentRenderStage::Background { position, size } => {
                    match self.style.background {
                        PreonBackground::Color(color) => pass.push(PreonShape::Rect {
                            position,
                            size,
                            color,
                            index: None,
                            radius: self.style.corner_radius,
                        }),
                        PreonBackground::Image(ref image) => pass.push(PreonShape::Rect {
                            position,
                            size,
                            color: PreonColor::TRANSPARENT_BLACK,
                            index: Some(image.index()),
                            radius: self.style.corner_radius,
                        }),
                        _ => (),
                    }
                }
                PreonComponentRenderStage::Foreground { position, size } => {
                    if !self.text.is_empty() {
                        pass.push(PreonShape::Text {
                            text_style: self.style.text_style.clone(),
                            color: self.style.foreground_color,
                            position,
                            size,
                            text: self.text.clone(),
                        })
                    }
                }
            }
        }

        #[cfg(feature = "debug")]
        {
            pass.push(PreonShape::Rect {
                position: component.get_outer_position(),
                size: component.get_outer_size(),
                color: PreonColor::from_hex("#c0687055"),
            });

            pass.push(PreonShape::Rect {
                position: component.get_border_position(),
                size: component.get_border_size(),
                color: PreonColor::from_hex("#c09b6855"),
            });

            pass.push(PreonShape::Rect {
                position: component.get_inner_position(),
                size: component.get_inner_size(),
                color: PreonColor::from_hex("#68c09355"),
            });

            pass.push(PreonShape::Rect {
                position: component.get_content_position(),
                size: component.get_content_size(),
                color: PreonColor::from_hex("#6891c055"),
            });
        }

        for child in self.children.iter_mut() {
            Self::render(child, pass)
        }
    }
}

impl Default for PreonComponent {
    fn default() -> Self {
        PreonComponent {
            style: PreonStyle {
                align_items: PreonAlignment::Start,
                cross_align_items: PreonAlignment::Center,
                background: PreonBackground::Color(PreonColor::WHITE),
                foreground_color: PreonColor::BLACK,
                ..Default::default()
            },
            children: Vec::new(),
            text: String::new(),
            inner_size: PreonVector::zero(),
            inner_position: PreonVector::zero(),
            id: None,
            id_lookup_cache: Vec::new(),
            mouse_events: false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PreonComponentRenderStage {
    Background {
        position: PreonVector<i32>,
        size: PreonVector<i32>,
    },
    Foreground {
        position: PreonVector<i32>,
        size: PreonVector<i32>,
    },
}

pub struct PreonComponentBuilder {
    pub stack: Vec<PreonComponent>,
}

#[allow(clippy::new_without_default)]
impl PreonComponentBuilder {
    pub fn new() -> PreonComponentBuilder {
        Self {
            stack: vec![PreonComponent {
                style: PreonStyle {
                    align_items: PreonAlignment::Start,
                    cross_align_items: PreonAlignment::Center,
                    ..Default::default()
                },
                ..Default::default()
            }],
        }
    }

    pub fn from_component(component: PreonComponent) -> PreonComponentBuilder {
        Self {
            stack: vec![component],
        }
    }

    pub fn with_child(&mut self, child: PreonComponent) -> &mut PreonComponentBuilder {
        self.current_mut().children.push(child);
        self
    }

    pub fn inherited_style(&self) -> PreonStyle {
        PreonStyle::inherit_from(&self.current().style)
    }

    pub fn receive_events(&mut self, receive_events: bool) -> &mut PreonComponentBuilder {
        self.stack.last_mut().unwrap().mouse_events = receive_events;
        self
    }

    pub fn id(&mut self, id: &str) -> &mut PreonComponentBuilder {
        self.stack.last_mut().unwrap().id = Some(id.to_string());
        self
    }

    pub fn id_string(&mut self, id: String) -> &mut PreonComponentBuilder {
        self.stack.last_mut().unwrap().id = Some(id);
        self
    }

    pub fn end(&mut self) -> &mut PreonComponentBuilder {
        let child = self.stack.pop().unwrap();
        self.with_child(child);
        self
    }

    pub fn build(&mut self) -> PreonComponent {
        self.stack.pop().unwrap()
    }

    pub fn current(&self) -> &PreonComponent {
        self.stack.last().unwrap()
    }

    pub fn current_mut(&mut self) -> &mut PreonComponent {
        self.stack.last_mut().unwrap()
    }
}
