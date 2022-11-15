use core::{fmt::Debug, str::FromStr};
use alloc::{vec::Vec, string::String, format, vec, borrow::ToOwned};

use crate::{
    rendering::{PreonRenderPass, PreonShape},
    size,
    types::{PreonAlignment, PreonBorder, PreonBox, PreonColor, PreonVector},
    log,
};

pub mod hbox;
pub mod vbox;
pub mod panel;
pub mod label;
pub mod static_texture;

pub use hbox::*;
pub use vbox::*;
pub use panel::*;
pub use label::*;
pub use static_texture::*;

#[derive(Debug, Clone)]
pub enum PreonComponentStack<T: PreonCustomComponentStack> {
    Custom(T),
    Dummy,
    Label {
        // <-- Largest item, making the size of this enum 32 bytes :/
        text: String,
        text_settings: u64,
    },
    StaticTexture {
        texture_index: usize,
    },
    Panel {
        color: PreonColor,
    },
    HBox {
        align: PreonAlignment,
        cross_align: PreonAlignment,
    },
    VBox {
        align: PreonAlignment,
        cross_align: PreonAlignment,
    },
}

/// A UI component
#[derive(Debug, Clone)]
pub struct PreonComponentStorage<T: PreonCustomComponentStack> {
    pub children: Vec<PreonComponentStorage<T>>,
    pub model: PreonBox,
    pub data: PreonComponentStack<T>,
    pub inner_size: PreonVector<i32>,
    pub inner_position: PreonVector<i32>,
    pub index_updates: Vec<isize>,
}

impl<T: PreonCustomComponentStack> PreonComponentStorage<T> {
    pub fn new(component: PreonComponentStack<T>) -> PreonComponentStorage<T> {
        PreonComponentStorage {
            children: Vec::new(),
            model: PreonBox::initial(),
            data: component,
            inner_size: PreonVector::zero(),
            inner_position: PreonVector::zero(),
            index_updates: Vec::new(),
        }
    }

    pub fn print_tree(&self, indent_level: usize) -> String {
        let indents = String::from("    ").repeat(indent_level);
        let mut children_strings = String::new();

        self.children
            .iter()
            .map(|c| c.print_tree(indent_level + 1))
            .collect::<Vec<String>>()
            .drain(..)
            .for_each(|s| children_strings.push_str(&format!("\n{}", s)));

        let (name, attributes) = T::display(&self.data);

        let mut extra = String::new();
        if !attributes.is_empty() {
            extra.push_str(&format!("\n{}  {}", indents, attributes));
        }

        let model = format!("{}", self.model);
        if !model.is_empty() {
            extra.push_str(&format!("\n{}  {}", indents, model))
        }

        format!("{}- {}{}{}", indents, name, extra, children_strings,)
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

    pub fn get_child_ref_recursive(&self, path: &[usize]) -> &PreonComponentStorage<T> {
        let mut _path = path.to_owned();
        let index = _path.pop().unwrap();

        if path.len() == 1 {
            self.get_child_ref(index)
        } else {
            self.get_child_ref(index).get_child_ref_recursive(&_path)
        }
    }

    pub fn get_child_ref_mut_recursive(&mut self, path: &[usize]) -> &mut PreonComponentStorage<T> {
        let mut _path = path.to_owned();
        let index = _path.pop().unwrap();

        if path.len() == 1 {
            self.get_child_ref_mut(index)
        } else {
            self.get_child_ref_mut(index)
                .get_child_ref_mut_recursive(&_path)
        }
    }

    pub fn get_child_ref(&self, id: usize) -> &PreonComponentStorage<T> {
        self.children
            .get(id)
            .unwrap()
    }

    pub fn get_child_ref_mut(&mut self, id: usize) -> &mut PreonComponentStorage<T> {
        self.children
            .get_mut(id)
            .unwrap()
    }

    pub fn add_child(&mut self, child: PreonComponentStorage<T>) {
        self.children.push(child);
    }

    pub fn insert_child(&mut self, id: usize, child: PreonComponentStorage<T>) {
        self.children.insert(id, child);
    }

    pub fn remove_child(&mut self, id: usize) {
        self.children.remove(id);
    }

    #[inline(always)]
    pub fn set_content_position(&mut self, new_position: PreonVector<i32>) {
        self.inner_position = new_position - self.model.padding.top_left();
    }

    #[inline(always)]
    pub fn get_content_position(&self) -> PreonVector<i32> {
        self.inner_position + self.model.padding.top_left()
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
        self.inner_position = new_position + self.model.border.top_left();
    }

    #[inline(always)]
    pub fn get_border_position(&self) -> PreonVector<i32> {
        self.inner_position - self.model.border.top_left()
    }

    #[inline(always)]
    pub fn set_outer_position(&mut self, new_position: PreonVector<i32>) {
        self.inner_position =
            new_position + self.model.border.top_left() + self.model.margin.top_left();
    }

    #[inline(always)]
    pub fn get_outer_position(&self) -> PreonVector<i32> {
        self.inner_position - self.model.border.top_left() - self.model.margin.top_left()
    }

    #[inline(always)]
    pub fn set_content_size(&mut self, new_size: PreonVector<i32>) {
        self.inner_size = new_size + self.model.padding;
    }

    #[inline(always)]
    pub fn get_content_size(&self) -> PreonVector<i32> {
        self.get_inner_size() - self.model.padding
    }

    #[inline(always)]
    pub fn set_inner_size(&mut self, new_size: PreonVector<i32>) {
        self.inner_size = new_size;
    }

    #[inline(always)]
    pub fn get_inner_size(&self) -> PreonVector<i32> {
        PreonVector::new(
            self.inner_size.x.max(self.model.min_size.x),
            self.inner_size.y.max(self.model.min_size.y),
        )
    }

    #[inline(always)]
    pub fn set_border_size(&mut self, new_size: PreonVector<i32>) {
        self.inner_size = new_size - self.model.border
    }

    #[inline(always)]
    pub fn get_border_size(&self) -> PreonVector<i32> {
        self.get_inner_size() + self.model.border
    }

    #[inline(always)]
    pub fn set_outer_size(&mut self, new_size: PreonVector<i32>) {
        self.inner_size = new_size - self.model.margin - self.model.border;
    }

    #[inline(always)]
    pub fn get_outer_size(&self) -> PreonVector<i32> {
        self.get_inner_size() + self.model.border + self.model.margin
    }

    #[inline(always)]
    pub fn set_content_size_x(&mut self, new_x: i32) {
        self.set_inner_size_x(new_x + self.model.padding.x());
    }

    #[inline(always)]
    pub fn set_inner_size_x(&mut self, new_x: i32) {
        self.inner_size.x = new_x;
    }

    #[inline(always)]
    pub fn set_border_size_x(&mut self, new_x: i32) {
        self.set_inner_size_x(new_x - self.model.margin.x());
    }

    #[inline(always)]
    pub fn set_outer_size_x(&mut self, new_x: i32) {
        self.set_inner_size_x(new_x - self.model.margin.x() - self.model.border.x());
    }

    #[inline(always)]
    pub fn set_content_size_y(&mut self, new_y: i32) {
        self.set_inner_size_y(new_y + self.model.padding.y());
    }

    #[inline(always)]
    pub fn set_inner_size_y(&mut self, new_y: i32) {
        self.inner_size.y = new_y;
    }

    #[inline(always)]
    pub fn set_border_size_y(&mut self, new_y: i32) {
        self.set_inner_size_y(new_y - self.model.margin.y());
    }

    #[inline(always)]
    pub fn set_outer_size_y(&mut self, new_y: i32) {
        self.set_inner_size_y(new_y - self.model.margin.y() - self.model.border.y());
    }
}

impl<T: PreonCustomComponentStack> Default for PreonComponentStorage<T> {
    fn default() -> Self {
        Self::new(PreonComponentStack::VBox {
            align: PreonAlignment::Start,
            cross_align: PreonAlignment::Center,
        })
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
    Border {
        position: PreonVector<i32>,
        size: PreonVector<i32>,
        width: PreonBorder,
    },
}

pub trait PreonCustomComponentStack: Debug + Sized {
    fn custom_layout(_comp: &mut PreonComponentStorage<Self>) {}
    fn custom_render(
        _stage: PreonComponentRenderStage,
        _component: &mut PreonComponentStorage<Self>,
        _pass: &mut PreonRenderPass,
    ) {
    }
    fn custom_display(_data: &Self) -> (String, String) {
        (String::new(), String::new())
    }

    fn display(data: &PreonComponentStack<Self>) -> (String, String) {
        let (name, attributes) = match *data {
            PreonComponentStack::Custom(ref d) => return Self::custom_display(d),
            PreonComponentStack::Dummy => ("Dummy", String::new()),
            PreonComponentStack::Label {
                ref text,
                text_settings: font_index,
            } => ("Label", format!("font: {}, text: \"{}\"", font_index, text)),
            PreonComponentStack::StaticTexture { texture_index } => {
                ("StaticTexture", format!("index: {}", texture_index))
            }
            PreonComponentStack::Panel { color } => ("Panel", format!("color: \"{}\"", color)),
            PreonComponentStack::HBox { align, cross_align } => (
                "HBox",
                format!("align: {}, cross_align: {}", align, cross_align),
            ),
            PreonComponentStack::VBox { align, cross_align } => (
                "VBox",
                format!("align: {}, cross_align: {}", align, cross_align),
            ),
        };

        (String::from_str(name).unwrap(), attributes)
    }

    fn layout(component: &mut PreonComponentStorage<Self>) {
        component.index_updates.clear();

        match component.data {
            PreonComponentStack::Custom(_) => Self::custom_layout(component),
            PreonComponentStack::Panel { .. } => panel::layout(component),
            PreonComponentStack::VBox { align, cross_align } => vbox::layout(component, align, cross_align),
            PreonComponentStack::HBox { align, cross_align } => hbox::layout(component, align, cross_align),
            _ => {}
        }

        for child in component.children.iter_mut() {
            Self::layout(child);
        }
    }

    fn render(component: &mut PreonComponentStorage<Self>, pass: &mut PreonRenderPass) {
        let stages = [
            PreonComponentRenderStage::Border {
                position: component.get_border_position(),
                size: component.get_border_size(),
                width: component.model.border,
            },
            PreonComponentRenderStage::Background {
                position: component.get_inner_position(),
                size: component.get_inner_size(),
            },
            PreonComponentRenderStage::Foreground {
                position: component.get_content_position(),
                size: component.get_content_size(),
            },
        ];

        for stage in stages {
            if let PreonComponentStack::Custom(_) = component.data {
                Self::custom_render(stage, component, pass);
            }

            match stage {
                PreonComponentRenderStage::Background { position, size } => match component.data {
                    PreonComponentStack::Panel { color } => pass.push(PreonShape::Rect {
                        position,
                        size,
                        color,
                    }),
                    PreonComponentStack::StaticTexture { texture_index } => {
                        pass.push(PreonShape::StaticTexture {
                            position,
                            size,
                            index: texture_index,
                        })
                    }
                    _ => {}
                },
                PreonComponentRenderStage::Foreground { position, size } => match component.data {
                    PreonComponentStack::Label {
                        ref text,
                        text_settings,
                    } => pass.push(PreonShape::Text {
                        text_settings,
                        position,
                        size,
                        text: text.clone(),
                    }),
                    _ => {}
                },
                PreonComponentRenderStage::Border { .. } => (),
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

        for child in component.children.iter_mut() {
            Self::render(child, pass)
        }
    }
}

pub struct PreonComponentBuilder<T: PreonCustomComponentStack> {
    stack: Vec<PreonComponentStorage<T>>,
}

#[allow(clippy::new_without_default)]
impl<T: PreonCustomComponentStack> PreonComponentBuilder<T> {
    pub fn new() -> PreonComponentBuilder<T> {
        Self {
            stack: vec![PreonComponentStorage {
                data: PreonComponentStack::VBox {
                    align: PreonAlignment::Start,
                    cross_align: PreonAlignment::Center,
                },
                ..Default::default()
            }],
        }
    }

    pub fn new_from(component: PreonComponentStack<T>) -> PreonComponentBuilder<T> {
        Self {
            stack: vec![PreonComponentStorage {
                data: component,
                ..Default::default()
            }],
        }
    }

    pub fn with_margin(&mut self, margin: PreonBorder) -> &mut PreonComponentBuilder<T> {
        self.current_mut().model.margin = margin;
        self
    }

    pub fn with_padding(&mut self, padding: PreonBorder) -> &mut PreonComponentBuilder<T> {
        self.current_mut().model.padding = padding;
        self
    }

    pub fn with_min_size(&mut self, x: i32, y: i32) -> &mut PreonComponentBuilder<T> {
        self.current_mut().model.min_size = PreonVector::new(x, y);
        self
    }

    pub fn with_border(&mut self, border: PreonBorder) -> &mut PreonComponentBuilder<T> {
        self.current_mut().model.border = border;
        self
    }

    pub fn fit_children(&mut self) -> &mut PreonComponentBuilder<T> {
        self.current_mut().model.size_flags |= size::FIT;
        self
    }

    pub fn fit_children_horizontally(&mut self) -> &mut PreonComponentBuilder<T> {
        self.current_mut().model.size_flags |= size::horizontal::FIT;
        self
    }

    pub fn fit_children_vertically(&mut self) -> &mut PreonComponentBuilder<T> {
        self.current_mut().model.size_flags |= size::vertical::FIT;
        self
    }

    pub fn expand(&mut self) -> &mut PreonComponentBuilder<T> {
        self.current_mut().model.size_flags |= size::EXPAND;
        self
    }

    pub fn expand_horizontally(&mut self) -> &mut PreonComponentBuilder<T> {
        self.current_mut().model.size_flags |= size::horizontal::EXPAND;
        self
    }

    pub fn expand_vertically(&mut self) -> &mut PreonComponentBuilder<T> {
        self.current_mut().model.size_flags |= size::vertical::EXPAND;
        self
    }

    pub fn with_child(&mut self, child: PreonComponentStorage<T>) -> &mut PreonComponentBuilder<T> {
        self.current_mut().children.push(child);        
        self
    }

    pub fn store_index(&mut self, reference: &mut usize) -> &mut PreonComponentBuilder<T> {
        *reference = self.get_index();

        self
    }

    pub fn store_path(&mut self, reference: &mut Vec<usize>) -> &mut PreonComponentBuilder<T> {
        reference.clear();
        reference.truncate(self.stack.len());
        reference.shrink_to_fit();

        let mut _stack: Vec<PreonComponentStorage<T>> = Vec::with_capacity(self.stack.capacity());

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

    pub fn with_mut<F>(&mut self, callback: F) -> &mut PreonComponentBuilder<T>
    where
        F: FnOnce(&mut PreonComponentStorage<T>),
    {
        callback(self.stack.last_mut().unwrap());
        self
    }

    pub fn end(&mut self) -> &mut PreonComponentBuilder<T> {
        let child = self.stack.pop().unwrap();
        self.with_child(child);
        self
    }

    pub fn build(&mut self) -> PreonComponentStorage<T> {
        self.stack.pop().unwrap()
    }

    fn current(&self) -> &PreonComponentStorage<T> {
        self.stack.last().unwrap()
    }

    fn current_mut(&mut self) -> &mut PreonComponentStorage<T> {
        self.stack.last_mut().unwrap()
    }
}

#[derive(Debug, Copy, Clone)]
pub enum NoCustomComponents {}
impl PreonCustomComponentStack for NoCustomComponents {}