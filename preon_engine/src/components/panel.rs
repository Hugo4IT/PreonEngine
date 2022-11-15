use crate::{types::PreonColor, style::{PreonBackground, PreonStyle}, layout::PreonLayout};

use super::{PreonComponent, PreonComponentBuilder};

pub trait AddPanel {
    fn start_panel(&mut self, color: PreonColor) -> &mut PreonComponentBuilder;
    fn empty_panel(&mut self, color: PreonColor) -> &mut PreonComponentBuilder;
    fn start_panel_hex(&mut self, hex_color: &'static str) -> &mut PreonComponentBuilder;
    fn empty_panel_hex(&mut self, hex_color: &'static str) -> &mut PreonComponentBuilder;
    fn panel_color(&mut self, color: PreonColor) -> &mut PreonComponentBuilder;
}

impl AddPanel for PreonComponentBuilder {
    fn start_panel(&mut self, color: PreonColor) -> &mut PreonComponentBuilder {
        self.stack.push(PreonComponent {
            style: PreonStyle {
                background: PreonBackground::Color(color),
                layout: PreonLayout::Container,
                ..Default::default()
            },
            ..Default::default()
        });

        self
    }

    fn empty_panel(&mut self, color: PreonColor) -> &mut PreonComponentBuilder {
        self.start_panel(color).end()
    }

    fn start_panel_hex(&mut self, hex_color: &'static str) -> &mut PreonComponentBuilder {
        self.start_panel(PreonColor::from_hex(hex_color))
    }

    fn empty_panel_hex(&mut self, hex_color: &'static str) -> &mut PreonComponentBuilder {
        self.start_panel_hex(hex_color).expand().end()
    }

    fn panel_color(&mut self, in_color: PreonColor) -> &mut PreonComponentBuilder {
        self.current_mut().style.background = PreonBackground::Color(in_color);
        self
    }
}

pub(super) fn layout(
    component: &mut PreonComponent,
) {
    let position = component.get_content_position();
    let size = component.get_content_size();
    for child in component.children.iter_mut() {
        child.set_outer_position(position);
        child.set_outer_size(size);
    }
}