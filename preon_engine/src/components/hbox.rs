use crate::{types::PreonAlignment, style::PreonStyle, layout::PreonLayout};

use super::{PreonComponent, PreonComponentBuilder};

pub trait PreonComponentBuilderHBoxExtension {
    fn start_hbox(&mut self) -> &mut PreonComponentBuilder;
    fn empty_hbox(&mut self) -> &mut PreonComponentBuilder;
}

impl PreonComponentBuilderHBoxExtension for PreonComponentBuilder {
    fn start_hbox(&mut self) -> &mut PreonComponentBuilder {
        self.stack.push(PreonComponent {
            style: PreonStyle {
                layout: PreonLayout::Columns,
                align_items: PreonAlignment::Start,
                cross_align_items: PreonAlignment::Center,
                ..Default::default()
            },
            ..Default::default()
        });

        self
    }

    fn empty_hbox(&mut self) -> &mut PreonComponentBuilder {
        self.start_hbox().end()
    }
}