use crate::{components::PreonComponent, rendering::PreonImage, style::{PreonStyle, PreonBackground}};

use super::PreonComponentBuilder;

pub trait PreonComponentBuilderStaticTextureExtension {
    fn start_static_texture(&mut self, image: &PreonImage) -> &mut PreonComponentBuilder;
}

impl PreonComponentBuilderStaticTextureExtension for PreonComponentBuilder {
    fn start_static_texture(&mut self, image: &PreonImage) -> &mut PreonComponentBuilder {
        self.stack.push(PreonComponent {
            style: PreonStyle {
                background: PreonBackground::Image(image),
                ..Default::default()
            },
            ..Default::default()
        });

        self
    }
}