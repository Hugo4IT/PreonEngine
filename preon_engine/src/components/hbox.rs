use crate::{types::{PreonAlignment, PreonVector}, size, components::PreonComponentStack, log};

use super::{PreonCustomComponentStack, PreonComponentStorage, PreonComponentBuilder};

pub trait AddHBox<T: PreonCustomComponentStack> {
    fn start_hbox(self) -> PreonComponentBuilder<T>;
    fn empty_hbox(self) -> PreonComponentBuilder<T>;
}

impl<T: PreonCustomComponentStack> AddHBox<T> for PreonComponentBuilder<T> {
    fn start_hbox(mut self) -> PreonComponentBuilder<T> {
        log::info!("start hbox");

        self.stack.push(PreonComponentStorage {
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

pub(super) fn layout<T: PreonCustomComponentStack>(
    component: &mut PreonComponentStorage<T>,
    align: PreonAlignment,
    cross_align: PreonAlignment,
) {
    let mut height = 0;
    let mut width = 0;
    let mut expanding_children = 0;
    let mut leftover_width = 0;

    // Gather some data on the children
    for child in component.children.iter() {
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

    for child in component.children.iter_mut() {
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
                    log::error!("HBox CrossAlignment doesn't support Spread (defaulting to Start)");
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