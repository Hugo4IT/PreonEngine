use core::fmt::Debug;
use alloc::{vec::Vec, string::String, vec, borrow::ToOwned};

use crate::{
    rendering::{PreonRenderPass, PreonShape},
    types::{PreonAlignment, PreonVector, PreonColor},
    style::{PreonStyle, PreonBackground, PreonForeground},
    layout::{
        PreonLayout,
        rows::PreonRowsLayoutProvider,
        columns::PreonColumnsLayoutProvider,
        container::PreonContainerLayoutProvider
    },
};

pub mod hbox;
pub mod vbox;
pub mod panel;
pub mod label;
pub mod static_texture;

/// A UI component
#[derive(Debug, Clone)]
pub struct PreonComponent {
    pub children: Vec<PreonComponent>,
    pub style: PreonStyle,
    pub text: String,
    pub inner_size: PreonVector<i32>,
    pub inner_position: PreonVector<i32>,
    pub index_updates: Vec<isize>,
}

impl PreonComponent {
    pub fn new() -> PreonComponent {
        PreonComponent {
            children: Vec::new(),
            style: PreonStyle::default(),
            text: String::new(),
            inner_size: PreonVector::zero(),
            inner_position: PreonVector::zero(),
            index_updates: Vec::new(),
        }
    }

    pub fn validate(&mut self, path: &mut Vec<usize>) {
        let mut current = self;
        path.reverse();

        'outer: for i in path.iter_mut() {
            for change in current.index_updates.iter() {
                if *i >= (*change).abs() as usize {
                    if *change < 0 {
                        *i -= 1;
                    } else {
                        *i += 1;
                    }

                    break 'outer;
                }
            }

            current = current.get_child_ref_mut(*i);
        }

        path.reverse();
    }

    pub fn get_child_ref_recursive(&self, path: &[usize]) -> &PreonComponent {
        let mut _path = path.to_owned();
        let index = _path.pop().unwrap();

        if path.len() == 1 {
            self.get_child_ref(index)
        } else {
            self.get_child_ref(index).get_child_ref_recursive(&_path)
        }
    }

    pub fn get_child_ref_mut_recursive(&mut self, path: &[usize]) -> &mut PreonComponent {
        let mut _path = path.to_owned();
        let index = _path.pop().unwrap();

        if path.len() == 1 {
            self.get_child_ref_mut(index)
        } else {
            self.get_child_ref_mut(index)
                .get_child_ref_mut_recursive(&_path)
        }
    }

    pub fn get_child_ref(&self, id: usize) -> &PreonComponent {
        self.children
            .get(id)
            .unwrap()
    }

    pub fn get_child_ref_mut(&mut self, id: usize) -> &mut PreonComponent {
        self.children
            .get_mut(id)
            .unwrap()
    }

    pub fn add_child(&mut self, child: PreonComponent) {
        self.children.push(child);
    }

    pub fn insert_child(&mut self, id: usize, child: PreonComponent) {
        self.children.insert(id, child);
    }

    pub fn remove_child(&mut self, id: usize) {
        self.children.remove(id);
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
    pub fn set_inner_position(&mut self, new_position: PreonVector<i32>) {
        self.inner_position = new_position;
    }

    #[inline(always)]
    pub fn get_inner_position(&self) -> PreonVector<i32> {
        self.inner_position
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
    pub fn set_outer_position(&mut self, new_position: PreonVector<i32>) {
        self.inner_position =
            new_position + self.style.border.top_left() + self.style.margin.top_left();
    }

    #[inline(always)]
    pub fn get_outer_position(&self) -> PreonVector<i32> {
        self.inner_position - self.style.border.top_left() - self.style.margin.top_left()
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

        self.index_updates.clear();

        match self.style.layout {
            PreonLayout::Rows => PreonRowsLayoutProvider::layout(self),
            PreonLayout::Columns => PreonColumnsLayoutProvider::layout(self),
            PreonLayout::Container => PreonContainerLayoutProvider::layout(self),
        }

        for child in self.children.iter_mut() {
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
                PreonComponentRenderStage::Background { position, size } => match self.style.background {
                    PreonBackground::Color(color) => pass.push(PreonShape::Rect {
                        position,
                        size,
                        color,
                        index: None,
                        radius: self.style.corner_radius,
                    }),
                    PreonBackground::Image(image) => pass.push(PreonShape::Rect {
                        position,
                        size,
                        color: PreonColor::TRANSPARENT_BLACK,
                        index: Some(image.index()),
                        radius: self.style.corner_radius,
                    }),
                    _ => (),
                },
                PreonComponentRenderStage::Foreground { position, size } => if !self.text.is_empty() {
                    pass.push(PreonShape::Text {
                        text_style: self.style.text_style,
                        color: match self.style.foreground {
                            PreonForeground::Color(color) => color,
                        },
                        position,
                        size,
                        text: self.text.clone(),
                    })
                },
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
                ..Default::default()
            },
            children: Vec::new(),
            text: String::new(),
            inner_size: PreonVector::zero(),
            inner_position: PreonVector::zero(),
            index_updates: Vec::new(),
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
    stack: Vec<PreonComponent>,
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

    pub fn with_child(&mut self, child: PreonComponent) -> &mut PreonComponentBuilder {
        self.current_mut().children.push(child);        
        self
    }

    pub fn store_index(&mut self, reference: &mut usize) -> &mut PreonComponentBuilder {
        *reference = self.get_index();

        self
    }

    pub fn store_path(&mut self, reference: &mut Vec<usize>) -> &mut PreonComponentBuilder {
        reference.clear();
        reference.truncate(self.stack.len());
        reference.shrink_to_fit();

        let mut _stack: Vec<PreonComponent> = Vec::with_capacity(self.stack.capacity());

        for _ in 0..self.stack.len() {
            if self.stack.len() >= 2 {
                reference.push(self.get_index());
            }
            _stack.push(self.stack.pop().take().unwrap());
        }

        for _ in 0.._stack.len() {
            self.stack.push(_stack.pop().take().unwrap());
        }

        self
    }

    fn get_index(&mut self) -> usize {
        if self.stack.len() == 1 {
            return 0;
        }

        self.stack
            .get(self.stack.len() - 2)
            .unwrap()
            .children
            .len()
    }

    pub fn with_mut<F>(&mut self, callback: F) -> &mut PreonComponentBuilder
    where
        F: FnOnce(&mut PreonComponent),
    {
        callback(self.stack.last_mut().unwrap());
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

    pub fn current_mut(&mut self) -> &mut PreonComponent {
        self.stack.last_mut().unwrap()
    }
}