use crate::{components::PreonComponentStack, types::PreonColor, log};

use super::{PreonComponentStorage, PreonCustomComponentStack, PreonComponentBuilder};

pub trait AddPanel<T: PreonCustomComponentStack> {
    fn start_panel(&mut self, color: PreonColor) -> &mut PreonComponentBuilder<T>;
    fn empty_panel(&mut self, color: PreonColor) -> &mut PreonComponentBuilder<T>;
    fn start_panel_hex(&mut self, hex_color: &'static str) -> &mut PreonComponentBuilder<T>;
    fn empty_panel_hex(&mut self, hex_color: &'static str) -> &mut PreonComponentBuilder<T>;
    fn panel_color(&mut self, color: PreonColor) -> &mut PreonComponentBuilder<T>;
}

impl<T: PreonCustomComponentStack> AddPanel<T> for PreonComponentBuilder<T> {
    fn start_panel(&mut self, color: PreonColor) -> &mut PreonComponentBuilder<T> {
        self.stack.push(PreonComponentStorage {
            data: PreonComponentStack::Panel { color },
            ..Default::default()
        });

        self
    }

    fn empty_panel(&mut self, color: PreonColor) -> &mut PreonComponentBuilder<T> {
        self.start_panel(color).end()
    }

    fn start_panel_hex(&mut self, hex_color: &'static str) -> &mut PreonComponentBuilder<T> {
        self.start_panel(PreonColor::from_hex(hex_color))
    }

    fn empty_panel_hex(&mut self, hex_color: &'static str) -> &mut PreonComponentBuilder<T> {
        self.start_panel_hex(hex_color).expand().end()
    }

    fn panel_color(&mut self, in_color: PreonColor) -> &mut PreonComponentBuilder<T> {
        if let PreonComponentStack::Panel { ref mut color } = self.current_mut().data {
            *color = in_color;
        } else {
            panic!("")
        }

        self
    }
}

pub(super) fn layout<T: PreonCustomComponentStack>(
    component: &mut PreonComponentStorage<T>,
) {
    let position = component.get_content_position();
    let size = component.get_content_size();
    for child in component.children.iter_mut() {
        child.set_outer_position(position);
        child.set_outer_size(size);
    }
}