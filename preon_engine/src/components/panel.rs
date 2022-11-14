use crate::{components::PreonComponentStack, types::PreonColor, log};

use super::{PreonComponentStorage, PreonCustomComponentStack, PreonComponentBuilder};

pub trait AddPanel<T: PreonCustomComponentStack> {
    fn start_panel(self, color: PreonColor) -> PreonComponentBuilder<T>;
    fn empty_panel(self, color: PreonColor) -> PreonComponentBuilder<T>;
    fn start_panel_hex(self, hex_color: &'static str) -> PreonComponentBuilder<T>;
    fn empty_panel_hex(self, hex_color: &'static str) -> PreonComponentBuilder<T>;
    fn panel_color(self, color: PreonColor) -> PreonComponentBuilder<T>;
}

impl<T: PreonCustomComponentStack> AddPanel<T> for PreonComponentBuilder<T> {
    fn start_panel(mut self, color: PreonColor) -> PreonComponentBuilder<T> {
        log::info!("start panel");

        self.stack.push(PreonComponentStorage {
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
        log::info!("panel color: {}", in_color);

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

pub(super) fn layout<T: PreonCustomComponentStack>(
    component: &mut PreonComponentStorage<T>,
) {
    let position = component.get_content_position();
    let size = component.get_content_size();
    if let Some(children) = component.children.as_mut() {
        for child in children.iter_mut() {
            if let Some(child) = child.as_mut() {
                child.set_outer_position(position);
                child.set_outer_size(size);
            } else {
                log::error!("A child was not returned before PreonEngine::update()!")
            }
        }
    }
}