use core::str::FromStr;

use alloc::string::String;

use crate::{components::PreonComponent, style::{PreonStyle, PreonBackground, PreonTextStyle}, layout::PreonLayout, types::{PreonColor, PreonAlignment}};
 
use super::{PreonComponentBuilder, vbox::PreonComponentBuilderVBoxExtension};

pub trait PreonComponentBuilderButtonExtension {
    fn start_button(&mut self, text: String) -> &mut PreonComponentBuilder;
    fn start_button_str(&mut self, text: &'static str) -> &mut PreonComponentBuilder;
    fn empty_button(&mut self, text: String) -> &mut PreonComponentBuilder;
    fn empty_button_str(&mut self, text: &'static str) -> &mut PreonComponentBuilder;
}

impl PreonComponentBuilderButtonExtension for PreonComponentBuilder {
    fn start_button(&mut self, text: String) -> &mut PreonComponentBuilder {
        self.stack.push(PreonComponent {
            text,
            style: PreonStyle {
                layout: PreonLayout::Container,
                background: PreonBackground::Color(PreonColor::from_rgba8(0x37, 0x63, 0xF2, 0xFF)),
                foreground_color: PreonColor::WHITE,
                text_style: PreonTextStyle {
                    vertical_align: PreonAlignment::Center,
                    horizontal_align: PreonAlignment::Center,
                    ..self.inherited_style().text_style
                },
                ..self.inherited_style()
            },
            mouse_events: true,
            ..Default::default()
        });

        self
    }

    fn start_button_str(&mut self, text: &'static str) -> &mut PreonComponentBuilder {
        self.start_button(String::from_str(text).unwrap())
    }

    fn empty_button(&mut self, text: String) -> &mut PreonComponentBuilder {
        self.start_button(text).end()
    }

    fn empty_button_str(&mut self, text: &'static str) -> &mut PreonComponentBuilder {
        self.start_button_str(text).end()
    }
}