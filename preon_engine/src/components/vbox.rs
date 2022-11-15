use crate::{types::{PreonAlignment, PreonVector}, size, log, style::PreonStyle, layout::PreonLayout};

use super::{PreonComponent, PreonComponentBuilder};

pub trait AddVBox {
    fn start_vbox(&mut self) -> &mut PreonComponentBuilder;
    fn empty_vbox(&mut self) -> &mut PreonComponentBuilder;
}

impl AddVBox for PreonComponentBuilder {
    fn start_vbox(&mut self) -> &mut PreonComponentBuilder {
        self.stack.push(PreonComponent {
            style: PreonStyle {
                layout: PreonLayout::Rows,
                align_items: PreonAlignment::Start,
                cross_align_items: PreonAlignment::Center,
                ..Default::default()
            },
            ..Default::default()
        });

        self
    }

    fn empty_vbox(&mut self) -> &mut PreonComponentBuilder {
        self.start_vbox().end()
    }
}

pub(super) fn layout(
    component: &mut PreonComponent,
) {
    let mut height = 0;
    let mut width = 0;
    let mut expanding_children = 0;
    let mut leftover_height = 0;

    // Gather some data on the children
    for child in component.children.iter() {
        let s = child.get_outer_size();

        if child.style.has_flag(size::vertical::EXPAND) {
            height += child.style.min_size.y;
            expanding_children += 1;
        } else {
            height += s.y;
            leftover_height += s.y;
        }

        if !child.style.has_flag(size::horizontal::EXPAND) {
            width = width.max(s.x);
        } else {
            width = width.max(child.style.min_size.x);
        }
    }

    let position = component.get_content_position();
    let mut size = component.get_content_size();

    if component.style.has_flag(size::horizontal::FIT) && size.x < width {
        component.set_content_size_x(width);
    }
    if component.style.has_flag(size::vertical::FIT) && size.y < height {
        component.set_content_size_y(height);
    }

    size = component.get_content_size();

    // Correctly position everything
    let mut y = 0;

    for child in component.children.iter_mut() {
        if child.style.has_flag(size::vertical::EXPAND) {
            child.set_outer_size_y((size.y - leftover_height) / expanding_children);
        }
        if child.style.has_flag(size::horizontal::EXPAND) {
            child.set_outer_size_x(size.x);
        }

        let child_size = child.get_outer_size();

        let x_position: i32 = if child.style.has_flag(size::horizontal::EXPAND) {
            0
        } else {
            match component.style.cross_align_items {
                PreonAlignment::Start => 0,
                PreonAlignment::Center => size.x / 2 - child_size.x / 2,
                PreonAlignment::End => size.x - child_size.x,
                PreonAlignment::Spread => {
                    log::error!("VBox CrossAlignment doesn't support Spread (defaulting to Start)");
                    0
                }
            }
        };

        let y_position: i32 = if expanding_children > 0 {
            y
        } else {
            match component.style.align_items {
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