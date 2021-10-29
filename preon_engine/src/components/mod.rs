use std::any::Any;

use crate::{
    rendering::{PreonRenderPass, PreonShape},
    size,
    types::{PreonAlignment, PreonBorder, PreonBox, PreonColor, PreonVector},
};

#[derive(Debug, Clone)]
pub struct PreonComponent<T: PreonCustomComponentStack> {
    pub children: Option<Vec<PreonComponent<T>>>,
    pub model: PreonBox,
    pub data: PreonComponentStack<T>,
    pub inner_size: PreonVector<i32>,
    pub inner_position: PreonVector<i32>,
}

impl<T: PreonCustomComponentStack> PreonComponent<T> {
    pub fn new(component: PreonComponentStack<T>) -> PreonComponent<T> {
        PreonComponent {
            children: Some(Vec::new()),
            model: PreonBox::initial(),
            data: component,
            inner_size: PreonVector::zero(),
            inner_position: PreonVector::zero(),
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
        self.inner_position = new_position + self.model.border.top_left() + self.model.margin.top_left();
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
            self.inner_size.y.max(self.model.min_size.y)
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
        Self::new(PreonComponentStack::vbox_default())
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

pub trait PreonCustomComponentStack {
    fn custom_layout<T: PreonCustomComponentStack + Any + 'static>(comp: &mut PreonComponent<T>);
    fn custom_render<T: PreonCustomComponentStack + Any + 'static>(
        stage: PreonComponentRenderStage,
        component: &mut PreonComponent<T>,
        pass: &mut PreonRenderPass,
    );

    fn layout<T: PreonCustomComponentStack + Any + 'static>(mut component: &mut PreonComponent<T>) {
        if let Some(mut children) = component.children.take() {
            component.children = Some(
                children
                    .drain(..)
                    .map(|mut f| -> PreonComponent<T> {
                        T::layout(&mut f);
                        f
                    })
                    .collect::<Vec<PreonComponent<T>>>(),
            );
        }

        match component.data {
            PreonComponentStack::Custom(_) => T::custom_layout::<T>(&mut component),
            PreonComponentStack::RectComponent { .. } => {}
            PreonComponentStack::VBoxComponent { align, cross_align } => {
                if component.children.is_some() {
                    let mut children = component.children.take().unwrap();

                    let mut height = 0;
                    let mut width = 0;
                    let mut expanding_children = 0;
                    let mut leftover_height = 0;

                    // Gather some data on the children
                    children.iter().for_each(|child| {
                        let s = child.get_outer_size();
                        height += s.y;
                        width = width.max(s.x);
                        if child.model.has_flag(size::vertical::EXPAND) {
                            expanding_children += 1;
                        } else {
                            leftover_height += s.y;
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

                    children.iter_mut().for_each(|child| {
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

                        let y_position: i32 = match align {
                            PreonAlignment::Start => y,
                            PreonAlignment::Center => size.y / 2 - y / 2,
                            PreonAlignment::End => size.y - y,
                            PreonAlignment::Spread => {
                                let time = 1f32 / y as f32;
                                ((1f32 - time) * y as f32 + time * (size.y - y) as f32) as i32
                            },
                        };

                        child.set_outer_position(position + PreonVector::new(x_position, y_position));

                        y += child_size.y;
                    });

                    component.children = Some(children);
                }
            },
            PreonComponentStack::HBoxComponent { align, cross_align } => {
                if component.children.is_some() {
                    let mut children = component.children.take().unwrap();

                    let mut height = 0;
                    let mut width = 0;
                    let mut expanding_children = 0;
                    let mut leftover_height = 0;

                    // Gather some data on the children
                    children.iter().for_each(|child| {
                        let s = child.get_outer_size();
                        height += s.y;
                        width = width.max(s.x);
                        if child.model.has_flag(size::vertical::EXPAND) {
                            expanding_children += 1;
                        } else {
                            leftover_height += s.y;
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

                    children.iter_mut().for_each(|child| {
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

                        let y_position: i32 = match align {
                            PreonAlignment::Start => y,
                            PreonAlignment::Center => size.y / 2 - y / 2,
                            PreonAlignment::End => size.y - y,
                            PreonAlignment::Spread => {
                                let time = 1f32 / y as f32;
                                ((1f32 - time) * y as f32 + time * (size.y - y) as f32) as i32
                            },
                        };

                        child.set_outer_position(position + PreonVector::new(x_position, y_position));

                        y += child_size.y;
                    });

                    component.children = Some(children);
                }
            }
        }
    }

    fn render<T: PreonCustomComponentStack + 'static>(
        component: &mut PreonComponent<T>,
        pass: &mut PreonRenderPass,
    ) {
        #[cfg(feature = "debug")]
        {
            pass.push(PreonShape::Rect {
                position: component.get_outer_position(),
                size: component.get_outer_size(),
                color: PreonColor::from_hex("#c06870")
            });

            pass.push(PreonShape::Rect {
                position: component.get_border_position(),
                size: component.get_border_size(),
                color: PreonColor::from_hex("#c09b68")
            });

            pass.push(PreonShape::Rect {
                position: component.get_inner_position(),
                size: component.get_inner_size(),
                color: PreonColor::from_hex("#68c093")
            });

            pass.push(PreonShape::Rect {
                position: component.get_content_position(),
                size: component.get_content_size(),
                color: PreonColor::from_hex("#6891c0")
            });
        }

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
                PreonComponentStack::RectComponent { color } => pass.push(PreonShape::Rect {
                    position,
                    size,
                    color,
                }),
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

        if let Some(mut children) = component.children.take() {
            component.children = Some(
                children
                    .drain(..)
                    .map(|mut f| -> PreonComponent<T> {
                        T::render(&mut f, pass);
                        f
                    })
                    .collect::<Vec<PreonComponent<T>>>(),
            );
        }
    }
}

#[derive(Debug, Clone)]
pub enum PreonComponentStack<T: PreonCustomComponentStack> {
    Custom(T),
    RectComponent {
        color: PreonColor,
    },
    HBoxComponent {
        align: PreonAlignment,
        cross_align: PreonAlignment
    },
    VBoxComponent {
        align: PreonAlignment,
        cross_align: PreonAlignment,
    },
}

impl<T: PreonCustomComponentStack> PreonComponentStack<T> {
    pub fn vbox_default() -> PreonComponentStack<T> {
        Self::VBoxComponent {
            align: PreonAlignment::default(),
            cross_align: PreonAlignment::default(),
        }
    }

    pub fn hbox_default() -> PreonComponentStack<T> {
        Self::HBoxComponent {
            align: PreonAlignment::default(),
            cross_align: PreonAlignment::default(),
        }
    }

    pub fn rect_default() -> PreonComponentStack<T> {
        Self::RectComponent {
            color: PreonColor::from_hex("#ffffff"),
        }
    }
}
