use std::{any::Any, fmt::Debug};

use log::warn;

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

    pub fn get_child_recursive(&mut self, path: &Vec<usize>) -> PreonComponent<T> {
        let mut _path = path.clone();
        let index = _path.pop().unwrap();

        if path.len() == 1 {
            self.get_child(index)
        } else {
            self.get_child_ref_mut(index).get_child_recursive(&_path)
        }
    }

    pub fn get_child_ref_recursive(&mut self, path: &Vec<usize>) -> &PreonComponent<T> {
        let mut _path = path.clone();
        let index = _path.pop().unwrap();

        if path.len() == 1 {
            self.get_child_ref(index)
        } else {
            self.get_child_ref_mut(index)
                .get_child_ref_recursive(&_path)
        }
    }

    pub fn get_child_ref_mut_recursive(&mut self, path: &Vec<usize>) -> &mut PreonComponent<T> {
        let mut _path = path.clone();
        let index = _path.pop().unwrap();

        if path.len() == 1 {
            self.get_child_ref_mut(index)
        } else {
            self.get_child_ref_mut(index)
                .get_child_ref_mut_recursive(&_path)
        }
    }

    pub fn return_child_recursive(&mut self, child: PreonComponent<T>, path: &Vec<usize>) {
        let mut _path = path.clone();
        let index = _path.pop().unwrap();

        if path.len() == 1 {
            self.return_child(index, child)
        } else {
            self.get_child_ref_mut(index)
                .return_child_recursive(child, &_path)
        }
    }

    pub fn get_child(&mut self, id: usize) -> PreonComponent<T> {
        self.children
            .as_mut()
            .unwrap()
            .get_mut(id)
            .take()
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

    pub fn return_child(&mut self, id: usize, child: PreonComponent<T>) {
        let mut children = self.children.take().unwrap();
        children[id] = Some(child);
        self.children = Some(children);
    }

    pub fn add_child(&mut self, child: PreonComponent<T>) {
        if self.children.is_some() {
            let mut children = self.children.take().unwrap();
            children.push(Some(child));
            self.children = Some(children);
        } else {
            self.children = Some(vec![Some(child)]);
        }
    }

    pub fn insert_child(&mut self, id: usize, child: PreonComponent<T>) {
        if self.children.is_some() {
            let mut children = self.children.take().unwrap();
            children.insert(id, Some(child));
            self.children = Some(children);

            self.index_updates.insert(0, id as isize);
        } else {
            self.children = Some(vec![Some(child)]);

            #[cfg(debug_assertions)]
            if id > 0 {
                eprintln!(
                    "You're trying to add a child to component {:?} at index {}, but not enough children are present.",
                    self,
                    id
                );
            }
        }
    }

    pub fn remove_child(&mut self, id: usize) {
        if self.children.is_some() {
            let mut children = self.children.take().unwrap();
            children.remove(id);

            if children.len() == 0 {
                self.children = None;
            } else {
                self.children = Some(children);
            }

            self.index_updates.insert(0, -(id as isize));
        } else {
            eprintln!(
                "You're trying to remove a child at index {} from component {:?}, but no child was found at the specified index.",
                id,
                self
            );
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

pub trait PreonCustomComponentStack: Debug {
    fn custom_layout<T: PreonCustomComponentStack + Any + 'static>(comp: &mut PreonComponent<T>);
    fn custom_render<T: PreonCustomComponentStack + Any + 'static>(
        stage: PreonComponentRenderStage,
        component: &mut PreonComponent<T>,
        pass: &mut PreonRenderPass,
    );

    fn layout<T: PreonCustomComponentStack + Any + 'static>(mut component: &mut PreonComponent<T>) {
        component.index_updates.clear();

        match component.data {
            PreonComponentStack::Custom(_) => T::custom_layout::<T>(&mut component),
            PreonComponentStack::Panel { .. } => {
                if component.children.is_some() {
                    let mut children = component.children.take().unwrap();

                    component.children = Some(
                        children
                            .drain(..)
                            .map(|mut ch| {
                                let mut child = ch.take().unwrap();
                                child.set_outer_position(component.get_content_position());
                                child.set_outer_size(component.get_content_size());
                                Some(child)
                            })
                            .collect(),
                    );
                }
            }
            PreonComponentStack::VBox { align, cross_align } => {
                if let Some(mut children) = component.children.take() {
                    let mut height = 0;
                    let mut width = 0;
                    let mut expanding_children = 0;
                    let mut leftover_height = 0;

                    // Gather some data on the children
                    children.iter_mut().for_each(|ch| {
                        let child = ch.as_mut().unwrap();

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
                    });

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

                    children.iter_mut().for_each(|ch| {
                        let child = ch.as_mut().unwrap();

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
                               },
                           }
                        };

                        child.set_outer_position(position + PreonVector::new(x_position, y_position));

                        y += child_size.y;
                    });

                    component.children = Some(children);
                }
            }
            PreonComponentStack::HBox { align, cross_align } => {
                if let Some(mut children) = component.children.take() {
                    let mut height = 0;
                    let mut width = 0;
                    let mut expanding_children = 0;
                    let mut leftover_width = 0;

                    // Gather some data on the children
                    children.iter_mut().for_each(|ch| {
                        let child = ch.as_mut().unwrap();

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
                    });

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

                    children.iter_mut().for_each(|ch| {
                        let child = ch.as_mut().unwrap();

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
                                },
                            }
                        };

                        child.set_outer_position(position + PreonVector::new(x_position, y_position));

                        x += child_size.x;
                    });

                    component.children = Some(children);
                }
            }
            _ => {}
        }

        if let Some(mut children) = component.children.take() {
            for child in children.iter_mut() {
                if let Some(child) = child.as_mut() {
                    T::layout(child);
                }
            }

            component.children = Some(children);
        }
    }

    fn render<T: PreonCustomComponentStack + 'static>(
        component: &mut PreonComponent<T>,
        pass: &mut PreonRenderPass,
    ) {
        let mut stages = vec![
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

        stages.drain(..).for_each(|stage| match stage {
            PreonComponentRenderStage::Background { position, size } => match component.data {
                PreonComponentStack::Custom(_) => T::custom_render::<T>(stage, component, pass),
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
            PreonComponentRenderStage::Foreground { .. } => match component.data {
                PreonComponentStack::Custom(_) => T::custom_render::<T>(stage, component, pass),
                _ => {}
            },
            PreonComponentRenderStage::Border { .. } => match component.data {
                PreonComponentStack::Custom(_) => T::custom_render::<T>(stage, component, pass),
                _ => {}
            },
        });

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

        if let Some(mut children) = component.children.take() {
            component.children = Some(
                children
                    .drain(..)
                    .map(|mut f| {
                        let mut comp = f.take().unwrap();
                        T::render(&mut comp, pass);
                        Some(comp)
                    })
                    .collect::<Vec<Option<PreonComponent<T>>>>(),
            );
        }
    }
}

#[derive(Debug, Clone)]
pub enum PreonComponentStack<T: PreonCustomComponentStack> {
    Custom(T),
    Dummy,
    StaticLabel {
        textdb_index: usize,

        // 0101010101[0][1][01][01]
        //            |  |   |   |=> Horizontal alignment: 00 - Left, 01 - Center, 10 - Right, 11 - Spread
        //            |  |   |=> Vertical alignment: 00 - Top, 01 - Center, 10 - Bottom
        //            |  |=> Bold?
        //            |=> Italic?
        font_details: u16,
    },
    Label {
        text: String,
        font_index: usize,
    },
    StaticTexture {
        texture_index: usize,
    },
    Panel {
        color: PreonColor, // <-- Largest item, making the size of this enum 16 bytes :/
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
        let mut component = self.stack.pop().take().unwrap();
        component.model.margin = margin;
        self.stack.push(component);
        self
    }

    pub fn with_padding(mut self, padding: PreonBorder) -> PreonComponentBuilder<T> {
        let mut component = self.stack.pop().take().unwrap();
        component.model.padding = padding;
        self.stack.push(component);
        self
    }

    pub fn with_min_size(mut self, x: i32, y: i32) -> PreonComponentBuilder<T> {
        let mut component = self.stack.pop().take().unwrap();
        component.model.min_size = PreonVector::new(x, y);
        self.stack.push(component);
        self
    }

    pub fn with_border(mut self, border: PreonBorder) -> PreonComponentBuilder<T> {
        let mut component = self.stack.pop().take().unwrap();
        component.model.border = border;
        self.stack.push(component);
        self
    }

    pub fn fit_children(mut self) -> PreonComponentBuilder<T> {
        let mut component = self.stack.pop().take().unwrap();
        component.model.size_flags = component.model.size_flags | size::FIT;
        self.stack.push(component);
        self
    }

    pub fn fit_children_horizontally(mut self) -> PreonComponentBuilder<T> {
        let mut component = self.stack.pop().take().unwrap();
        component.model.size_flags = component.model.size_flags | size::horizontal::FIT;
        self.stack.push(component);
        self
    }

    pub fn fit_children_vertically(mut self) -> PreonComponentBuilder<T> {
        let mut component = self.stack.pop().take().unwrap();
        component.model.size_flags = component.model.size_flags | size::vertical::FIT;
        self.stack.push(component);
        self
    }

    pub fn expand(mut self) -> PreonComponentBuilder<T> {
        let mut component = self.stack.pop().take().unwrap();
        component.model.size_flags = component.model.size_flags | size::EXPAND;
        self.stack.push(component);
        self
    }

    pub fn expand_horizontally(mut self) -> PreonComponentBuilder<T> {
        let mut component = self.stack.pop().take().unwrap();
        component.model.size_flags = component.model.size_flags | size::horizontal::EXPAND;
        self.stack.push(component);
        self
    }

    pub fn expand_vertically(mut self) -> PreonComponentBuilder<T> {
        let mut component = self.stack.pop().take().unwrap();
        component.model.size_flags = component.model.size_flags | size::vertical::EXPAND;
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

    pub fn end(mut self) -> PreonComponentBuilder<T> {
        let child = self.stack.pop().unwrap();
        self.with_child(child)
    }

    pub fn build(mut self) -> PreonComponent<T> {
        self.stack.pop().unwrap()
    }
}

pub trait AddVBox<T: PreonCustomComponentStack> {
    fn start_vbox(self) -> PreonComponentBuilder<T>;
    fn empty_vbox(self) -> PreonComponentBuilder<T>;
}

impl<T: PreonCustomComponentStack> AddVBox<T> for PreonComponentBuilder<T> {
    fn start_vbox(mut self) -> PreonComponentBuilder<T> {
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
    fn start_panel(self, hex_color: &'static str) -> PreonComponentBuilder<T>;
    fn empty_panel(self, hex_color: &'static str) -> PreonComponentBuilder<T>;
}

impl<T: PreonCustomComponentStack> AddPanel<T> for PreonComponentBuilder<T> {
    fn start_panel(mut self, hex_color: &'static str) -> PreonComponentBuilder<T> {
        self.stack.push(PreonComponent {
            data: PreonComponentStack::Panel {
                color: PreonColor::from_hex(hex_color),
            },
            ..Default::default()
        });

        self
    }

    fn empty_panel(self, hex_color: &'static str) -> PreonComponentBuilder<T> {
        self.start_panel(hex_color).expand().end()
    }
}

pub trait AddStaticTexture<T: PreonCustomComponentStack> {
    fn start_static_texture(self, index: usize) -> PreonComponentBuilder<T>;
}

impl<T: PreonCustomComponentStack> AddStaticTexture<T> for PreonComponentBuilder<T> {
    fn start_static_texture(mut self, index: usize) -> PreonComponentBuilder<T> {
        self.stack.push(PreonComponent {
            data: PreonComponentStack::StaticTexture {
                texture_index: index,
            },
            ..Default::default()
        });

        self
    }
}

#[derive(Debug, Copy, Clone)]
pub enum NoCustomComponents {}

impl PreonCustomComponentStack for NoCustomComponents {
    fn custom_layout<T: PreonCustomComponentStack + Any + 'static>(_: &mut PreonComponent<T>) {}

    fn custom_render<T: PreonCustomComponentStack + Any + 'static>(
        _: PreonComponentRenderStage,
        _: &mut PreonComponent<T>,
        _: &mut PreonRenderPass,
    ) {
    }
}
