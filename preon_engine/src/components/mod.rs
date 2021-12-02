use std::{fmt::Debug, str::FromStr};

use log::info;

use crate::{
    rendering::{PreonRenderPass, PreonShape},
    size,
    types::{PreonAlignment, PreonBorder, PreonBox, PreonColor, PreonVector},
};

/// A UI component
#[derive(Debug, Clone)]
pub struct PreonComponent<T: PreonCustomComponentStack> {
    pub children: Option<Vec<Option<PreonComponent<T>>>>,
    pub model: PreonBox,
    pub data: PreonComponentStack<T>,
    pub inner_size: PreonVector<i32>,
    pub inner_position: PreonVector<i32>,
    pub index_updates: Vec<isize>,
}

impl<T: PreonCustomComponentStack> PreonComponent<T> {
    pub fn new(component: PreonComponentStack<T>) -> PreonComponent<T> {
        PreonComponent {
            children: None,
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

        if let Some(children) = self.children.as_ref() {
            children
                .iter()
                .map(|c| c.as_ref().unwrap().print_tree(indent_level + 1))
                .collect::<Vec<String>>()
                .drain(..)
                .for_each(|s| children_strings.push_str(&format!("\n{}", s)));
        }

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

    pub fn get_child_ref_recursive(&self, path: &[usize]) -> &PreonComponent<T> {
        let mut _path = path.to_owned();
        let index = _path.pop().unwrap();

        if path.len() == 1 {
            self.get_child_ref(index)
        } else {
            self.get_child_ref(index).get_child_ref_recursive(&_path)
        }
    }

    pub fn get_child_ref_mut_recursive(&mut self, path: &[usize]) -> &mut PreonComponent<T> {
        let mut _path = path.to_owned();
        let index = _path.pop().unwrap();

        if path.len() == 1 {
            self.get_child_ref_mut(index)
        } else {
            self.get_child_ref_mut(index)
                .get_child_ref_mut_recursive(&_path)
        }
    }

    pub fn get_child(&mut self, id: usize) -> PreonComponent<T> {
        self.children
            .as_mut()
            .unwrap()
            .get_mut(id)
            .unwrap()
            .take()
            .unwrap()
    }

    pub fn get_child_ref(&self, id: usize) -> &PreonComponent<T> {
        self.children
            .as_ref()
            .unwrap()
            .get(id)
            .unwrap()
            .as_ref()
            .unwrap()
    }

    pub fn get_child_ref_mut(&mut self, id: usize) -> &mut PreonComponent<T> {
        self.children
            .as_mut()
            .unwrap()
            .get_mut(id)
            .unwrap()
            .as_mut()
            .unwrap()
    }

    pub fn add_child(&mut self, child: PreonComponent<T>) {
        if let Some(children) = self.children.as_mut() {
            children.push(Some(child));
        } else {
            self.children.replace(vec![Some(child)]);
        }
    }

    pub fn insert_child(&mut self, id: usize, child: PreonComponent<T>) {
        if let Some(children) = self.children.as_mut() {
            children.insert(id, Some(child));
        } else {
            self.children.replace(vec![Some(child)]);
        }
    }

    pub fn remove_child(&mut self, id: usize) {
        if let Some(children) = self.children.as_mut() {
            children.remove(id);
        } else {
            eprintln!("component.remove_child was called, but the component had no children!")
        }
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

impl<T: PreonCustomComponentStack> Default for PreonComponent<T> {
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
    fn custom_layout(_comp: &mut PreonComponent<Self>) {}
    fn custom_render(
        _stage: PreonComponentRenderStage,
        _component: &mut PreonComponent<Self>,
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

    fn layout(component: &mut PreonComponent<Self>) {
        component.index_updates.clear();

        match component.data {
            PreonComponentStack::Custom(_) => Self::custom_layout(component),
            PreonComponentStack::Panel { .. } => {
                let position = component.get_content_position();
                let size = component.get_content_size();
                if let Some(children) = component.children.as_mut() {
                    for child in children.iter_mut() {
                        if let Some(child) = child.as_mut() {
                            child.set_outer_position(position);
                            child.set_outer_size(size);
                        } else {
                            eprintln!("A child was not returned before PreonEngine::update()!")
                        }
                    }
                }
            }
            PreonComponentStack::VBox { align, cross_align } => {
                if component.children.is_some() {
                    let mut height = 0;
                    let mut width = 0;
                    let mut expanding_children = 0;
                    let mut leftover_height = 0;

                    // Gather some data on the children
                    for child in component.children.as_ref().unwrap().iter() {
                        let child = child.as_ref().unwrap();
                        let s = child.get_outer_size();

                        if child.model.has_flag(size::vertical::EXPAND) {
                            height += child.model.min_size.y;
                            expanding_children += 1;
                        } else {
                            height += s.y;
                            leftover_height += s.y;
                        }

                        if !child.model.has_flag(size::horizontal::EXPAND) {
                            width = width.max(s.x);
                        } else {
                            width = width.max(child.model.min_size.x);
                        }
                    }

                    let position = component.get_content_position();
                    let mut size = component.get_content_size();

                    if component.model.has_flag(size::horizontal::FIT) && size.x < width {
                        component.set_content_size_x(width);
                    }
                    if component.model.has_flag(size::vertical::FIT) && size.y < height {
                        component.set_content_size_y(height);
                    }

                    size = component.get_content_size();

                    // Correctly position everything
                    let mut y = 0;

                    for child in component.children.as_mut().unwrap().iter_mut() {
                        let child = child.as_mut().unwrap();

                        if child.model.has_flag(size::vertical::EXPAND) {
                            child.set_outer_size_y((size.y - leftover_height) / expanding_children);
                        }
                        if child.model.has_flag(size::horizontal::EXPAND) {
                            child.set_outer_size_x(size.x);
                        }

                        let child_size = child.get_outer_size();

                        let x_position: i32 = if child.model.has_flag(size::horizontal::EXPAND) {
                            0
                        } else {
                            match cross_align {
                                PreonAlignment::Start => 0,
                                PreonAlignment::Center => size.x / 2 - child_size.x / 2,
                                PreonAlignment::End => size.x - child_size.x,
                                PreonAlignment::Spread => {
                                    eprintln!("VBox CrossAlignment doesn't support Spread (defaulting to Start)");
                                    0
                                }
                            }
                        };

                        let y_position: i32 = if expanding_children > 0 {
                            y
                        } else {
                            match align {
                                PreonAlignment::Start => y,
                                PreonAlignment::Center => size.y / 2 - height / 2 + y,
                                PreonAlignment::End => (size.y - height) + y,
                                PreonAlignment::Spread => {
                                    let time = 1f32 / y as f32;
                                    ((1f32 - time) * y as f32 + time * (size.y - y) as f32) as i32
                                }
                            }
                        };

                        child.set_outer_position(
                            position + PreonVector::new(x_position, y_position),
                        );

                        y += child_size.y;
                    }
                }
            }
            PreonComponentStack::HBox { align, cross_align } => {
                if component.children.is_some() {
                    let mut height = 0;
                    let mut width = 0;
                    let mut expanding_children = 0;
                    let mut leftover_width = 0;

                    // Gather some data on the children
                    for child in component.children.as_ref().unwrap().iter() {
                        let child = child.as_ref().unwrap();

                        let s = child.get_outer_size();

                        if child.model.has_flag(size::horizontal::EXPAND) {
                            width += child.model.min_size.x;
                            expanding_children += 1;
                        } else {
                            width += s.x;
                            leftover_width += s.x;
                        }

                        if !child.model.has_flag(size::vertical::EXPAND) {
                            height = height.max(s.y);
                        } else {
                            height = height.max(child.model.min_size.y);
                        }
                    }

                    let position = component.get_content_position();
                    let mut size = component.get_content_size();

                    if component.model.has_flag(size::horizontal::FIT) && size.x < width {
                        component.set_content_size_x(width);
                    }
                    if component.model.has_flag(size::vertical::FIT) && size.y < height {
                        component.set_content_size_y(height);
                    }

                    size = component.get_content_size();

                    // Correctly position everything
                    let mut x = 0;

                    for child in component.children.as_mut().unwrap().iter_mut() {
                        let child = child.as_mut().unwrap();

                        if child.model.has_flag(size::horizontal::EXPAND) {
                            child.set_outer_size_x((size.x - leftover_width) / expanding_children);
                        }
                        if child.model.has_flag(size::vertical::EXPAND) {
                            child.set_outer_size_y(size.y);
                        }

                        let child_size = child.get_outer_size();

                        let y_position: i32 = if child.model.has_flag(size::vertical::EXPAND) {
                            0
                        } else {
                            match cross_align {
                                PreonAlignment::Start => 0,
                                PreonAlignment::Center => size.y / 2 - child_size.y / 2,
                                PreonAlignment::End => size.y - child_size.y,
                                PreonAlignment::Spread => {
                                    eprintln!("HBox CrossAlignment doesn't support Spread (defaulting to Start)");
                                    0
                                }
                            }
                        };

                        let x_position: i32 = if expanding_children > 0 {
                            x
                        } else {
                            match align {
                                PreonAlignment::Start => x,
                                PreonAlignment::Center => size.x / 2 - width / 2 + x,
                                PreonAlignment::End => (size.x - width) + x,
                                PreonAlignment::Spread => {
                                    let time = 1f32 / x as f32;
                                    ((1f32 - time) * x as f32 + time * (size.x - x) as f32) as i32
                                }
                            }
                        };

                        child.set_outer_position(
                            position + PreonVector::new(x_position, y_position),
                        );

                        x += child_size.x;
                    }
                }
            }
            _ => {}
        }

        if let Some(children) = component.children.as_mut() {
            for child in children.iter_mut() {
                Self::layout(child.as_mut().unwrap());
            }
        }
    }

    fn render(component: &mut PreonComponent<Self>, pass: &mut PreonRenderPass) {
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
                        text_settings: font_index,
                    } => pass.push(PreonShape::Text {
                        font_index,
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

        if let Some(children) = component.children.as_mut() {
            for child in children.iter_mut() {
                Self::render(child.as_mut().unwrap(), pass)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum PreonComponentStack<T: PreonCustomComponentStack> {
    Custom(T),
    Dummy,
    Label { // <-- Largest item, making the size of this enum 32 bytes :/
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

pub struct PreonComponentBuilder<T: PreonCustomComponentStack> {
    stack: Vec<PreonComponent<T>>,
}

#[allow(clippy::new_without_default)]
impl<T: PreonCustomComponentStack> PreonComponentBuilder<T> {
    pub fn new() -> PreonComponentBuilder<T> {
        Self {
            stack: vec![PreonComponent {
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
            stack: vec![PreonComponent {
                data: component,
                ..Default::default()
            }],
        }
    }

    pub fn with_margin(mut self, margin: PreonBorder) -> PreonComponentBuilder<T> {
        info!("with margin: {}", margin);

        let mut component = self.stack.pop().take().unwrap();
        component.model.margin = margin;
        self.stack.push(component);
        self
    }

    pub fn with_padding(mut self, padding: PreonBorder) -> PreonComponentBuilder<T> {
        info!("with padding: {}", padding);

        let mut component = self.stack.pop().take().unwrap();
        component.model.padding = padding;
        self.stack.push(component);
        self
    }

    pub fn with_min_size(mut self, x: i32, y: i32) -> PreonComponentBuilder<T> {
        info!("with min_size: {}x{}", x, y);

        let mut component = self.stack.pop().take().unwrap();
        component.model.min_size = PreonVector::new(x, y);
        self.stack.push(component);
        self
    }

    pub fn with_border(mut self, border: PreonBorder) -> PreonComponentBuilder<T> {
        info!("with border: {}", border);

        let mut component = self.stack.pop().take().unwrap();
        component.model.border = border;
        self.stack.push(component);
        self
    }

    pub fn fit_children(mut self) -> PreonComponentBuilder<T> {
        info!("fit children");

        let mut component = self.stack.pop().take().unwrap();
        component.model.size_flags |= size::FIT;
        self.stack.push(component);
        self
    }

    pub fn fit_children_horizontally(mut self) -> PreonComponentBuilder<T> {
        info!("fit children horizontally");

        let mut component = self.stack.pop().take().unwrap();
        component.model.size_flags |= size::horizontal::FIT;
        self.stack.push(component);
        self
    }

    pub fn fit_children_vertically(mut self) -> PreonComponentBuilder<T> {
        info!("fit children vertically");

        let mut component = self.stack.pop().take().unwrap();
        component.model.size_flags |= size::vertical::FIT;
        self.stack.push(component);
        self
    }

    pub fn expand(mut self) -> PreonComponentBuilder<T> {
        info!("expand");

        let mut component = self.stack.pop().take().unwrap();
        component.model.size_flags |= size::EXPAND;
        self.stack.push(component);
        self
    }

    pub fn expand_horizontally(mut self) -> PreonComponentBuilder<T> {
        info!("expand horizontally");

        let mut component = self.stack.pop().take().unwrap();
        component.model.size_flags |= size::horizontal::EXPAND;
        self.stack.push(component);
        self
    }

    pub fn expand_vertically(mut self) -> PreonComponentBuilder<T> {
        info!("expand vertically");

        let mut component = self.stack.pop().take().unwrap();
        component.model.size_flags |= size::vertical::EXPAND;
        self.stack.push(component);
        self
    }

    pub fn with_child(mut self, child: PreonComponent<T>) -> PreonComponentBuilder<T> {
        let mut component = self.stack.pop().take().unwrap();

        if component.children.is_none() {
            component.children = Some(vec![Some(child)]);
        } else {
            let mut children = component.children.take().unwrap();
            children.push(Some(child));
            component.children = Some(children);
        }

        self.stack.push(component);
        self
    }

    pub fn store_index(mut self, reference: &mut usize) -> PreonComponentBuilder<T> {
        *reference = self.get_index();

        self
    }

    pub fn store_path(mut self, reference: &mut Vec<usize>) -> PreonComponentBuilder<T> {
        reference.clear();
        reference.truncate(self.stack.len());
        reference.shrink_to_fit();

        let mut _stack: Vec<PreonComponent<T>> = Vec::with_capacity(self.stack.capacity());

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
        let new_id: usize;

        if self.stack.len() == 1 {
            return 0;
        }

        let component = self.stack.pop().take().unwrap();
        let mut parent_component = self.stack.pop().take().unwrap();

        if let Some(children) = parent_component.children {
            new_id = children.len();
            parent_component.children = Some(children);
        } else {
            new_id = 0;
        }

        self.stack.push(parent_component);
        self.stack.push(component);

        new_id
    }

    pub fn with_mut<F>(mut self, callback: F) -> PreonComponentBuilder<T>
    where
        F: FnOnce(&mut PreonComponent<T>),
    {
        let mut component = self.stack.pop().unwrap();
        callback(&mut component);
        self.stack.push(component);
        self
    }

    pub fn end(mut self) -> PreonComponentBuilder<T> {
        info!("end");

        let child = self.stack.pop().unwrap();
        self.with_child(child)
    }

    pub fn build(mut self) -> PreonComponent<T> {
        info!("build");

        self.stack.pop().unwrap()
    }
}

pub trait AddVBox<T: PreonCustomComponentStack> {
    fn start_vbox(self) -> PreonComponentBuilder<T>;
    fn empty_vbox(self) -> PreonComponentBuilder<T>;
}

impl<T: PreonCustomComponentStack> AddVBox<T> for PreonComponentBuilder<T> {
    fn start_vbox(mut self) -> PreonComponentBuilder<T> {
        info!("start vbox");

        self.stack.push(PreonComponent {
            data: PreonComponentStack::VBox {
                align: PreonAlignment::Start,
                cross_align: PreonAlignment::Center,
            },
            ..Default::default()
        });

        self
    }

    fn empty_vbox(self) -> PreonComponentBuilder<T> {
        self.start_vbox().end()
    }
}

pub trait AddHBox<T: PreonCustomComponentStack> {
    fn start_hbox(self) -> PreonComponentBuilder<T>;
    fn empty_hbox(self) -> PreonComponentBuilder<T>;
}

impl<T: PreonCustomComponentStack> AddHBox<T> for PreonComponentBuilder<T> {
    fn start_hbox(mut self) -> PreonComponentBuilder<T> {
        info!("start hbox");

        self.stack.push(PreonComponent {
            data: PreonComponentStack::HBox {
                align: PreonAlignment::Start,
                cross_align: PreonAlignment::Center,
            },
            ..Default::default()
        });

        self
    }

    fn empty_hbox(self) -> PreonComponentBuilder<T> {
        self.start_hbox().end()
    }
}

pub trait AddPanel<T: PreonCustomComponentStack> {
    fn start_panel(self, color: PreonColor) -> PreonComponentBuilder<T>;
    fn empty_panel(self, color: PreonColor) -> PreonComponentBuilder<T>;
    fn start_panel_hex(self, hex_color: &'static str) -> PreonComponentBuilder<T>;
    fn empty_panel_hex(self, hex_color: &'static str) -> PreonComponentBuilder<T>;
    fn panel_color(self, color: PreonColor) -> PreonComponentBuilder<T>;
}

impl<T: PreonCustomComponentStack> AddPanel<T> for PreonComponentBuilder<T> {
    fn start_panel(mut self, color: PreonColor) -> PreonComponentBuilder<T> {
        info!("start panel");

        self.stack.push(PreonComponent {
            data: PreonComponentStack::Panel { color },
            ..Default::default()
        });

        self
    }

    fn empty_panel(self, color: PreonColor) -> PreonComponentBuilder<T> {
        self.start_panel(color).end()
    }

    fn start_panel_hex(self, hex_color: &'static str) -> PreonComponentBuilder<T> {
        self.start_panel(PreonColor::from_hex(hex_color))
    }

    fn empty_panel_hex(self, hex_color: &'static str) -> PreonComponentBuilder<T> {
        self.start_panel_hex(hex_color).expand().end()
    }

    fn panel_color(mut self, in_color: PreonColor) -> PreonComponentBuilder<T> {
        info!("panel color: {}", in_color);

        let mut component = self.stack.pop().unwrap();

        if let PreonComponentStack::Panel { ref mut color } = component.data {
            *color = in_color;
        } else {
            panic!("")
        }

        self.stack.push(component);
        self
    }
}

pub trait AddStaticTexture<T: PreonCustomComponentStack> {
    fn start_static_texture(self, index: usize) -> PreonComponentBuilder<T>;
}

impl<T: PreonCustomComponentStack> AddStaticTexture<T> for PreonComponentBuilder<T> {
    fn start_static_texture(mut self, index: usize) -> PreonComponentBuilder<T> {
        info!("start static texture: {}", index);

        self.stack.push(PreonComponent {
            data: PreonComponentStack::StaticTexture {
                texture_index: index,
            },
            ..Default::default()
        });

        self
    }
}

/// Human-readable text configuration
///
/// 64 bytes, full text configuration, best memory layout I could think of.
///
/// ```txt
/// 0000000000 0 0 00 00 00000000|00000000|00000000|00000000 0000000000000000
///            ¯ ¯ ¯¯ ¯¯ ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯ ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯
///            | | |  |   text color [u8; 4]                 font_index: u16
///            | | |  ^> vertical_align: PreonAlignment (2 bytes, 4 options)
///            | | ^> horizontal_align: PreonAlignment (2 bytes, 4 options)
///            | ^> italic: bool
///            ^> bold: bool
/// ```
#[derive(Debug, Clone, Copy)]
pub struct AddLabelConfig {
    pub font_index: u16,
    pub color_rgba8: PreonColor,
    pub bold: bool,
    pub italic: bool,
    pub vertical_align: PreonAlignment,
    pub horizontal_align: PreonAlignment,
}

impl AddLabelConfig {
    pub fn encode(&self) -> u64 {
        let mut result: [u8; 8] = [0u8; 8];

        let mut flags: u8 = 0u8;
        flags |= (self.bold as u8) << 5;
        flags |= (self.italic as u8) << 4;
        flags |= (self.vertical_align as u8) << 2;
        flags |= self.horizontal_align as u8;
        result[1] = flags;

        u64::from_le_bytes(result)
    }
}

pub trait AddLabel<T: PreonCustomComponentStack> {
    fn start_label(self, text: String) -> PreonComponentBuilder<T>;
    fn start_label_str(self, text: &'static str) -> PreonComponentBuilder<T>;
    fn empty_label(self, text: String) -> PreonComponentBuilder<T>;
    fn empty_label_str(self, text: &'static str) -> PreonComponentBuilder<T>;
    fn start_label_cfg(self, text: String, config: AddLabelConfig) -> PreonComponentBuilder<T>;
}

impl<T: PreonCustomComponentStack> AddLabel<T> for PreonComponentBuilder<T> {
    fn start_label(mut self, text: String) -> PreonComponentBuilder<T> {
        info!("start label: {}", text);

        self.stack.push(PreonComponent {
            data: PreonComponentStack::Label {
                text,
                text_settings: 0,
            },
            ..Default::default()
        });

        self
    }

    fn start_label_str(self, text: &'static str) -> PreonComponentBuilder<T> {
        self.start_label(String::from_str(text).unwrap())
    }

    fn empty_label(self, text: String) -> PreonComponentBuilder<T> {
        self.start_label(text).end()
    }

    fn empty_label_str(self, text: &'static str) -> PreonComponentBuilder<T> {
        self.start_label_str(text).end()
    }

    fn start_label_cfg(mut self, text: String, config: AddLabelConfig) -> PreonComponentBuilder<T> {
        info!("Start label with config {:?}", config);

        self.stack.push(PreonComponent {
            data: PreonComponentStack::Label {
                text,
                text_settings: config.encode()
            },
            ..Default::default()
        });

        self
    }
}

#[derive(Debug, Copy, Clone)]
pub enum NoCustomComponents {}
impl PreonCustomComponentStack for NoCustomComponents {}
