use crate::{types::PreonAlignment, style::PreonStyle, layout::PreonLayout};

use super::{PreonComponent, PreonComponentBuilder};

pub trait PreonComponentBuilderVBoxExtension {
    fn start_vbox(&mut self) -> &mut PreonComponentBuilder;
    fn empty_vbox(&mut self) -> &mut PreonComponentBuilder;
}

impl PreonComponentBuilderVBoxExtension for PreonComponentBuilder {
    fn start_vbox(&mut self) -> &mut PreonComponentBuilder {
        self.stack.push(PreonComponent {
            style: PreonStyle {
                layout: PreonLayout::Rows,
                align_items: PreonAlignment::Start,
                cross_align_items: PreonAlignment::Start,
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