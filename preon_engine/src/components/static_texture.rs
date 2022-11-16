use crate::{components::PreonComponent, style::{PreonStyle, PreonBackground, image::PreonImage}};

use super::PreonComponentBuilder;

pub trait PreonComponentBuilderStaticTextureExtension {
    fn start_static_texture(&mut self, index: usize) -> &mut PreonComponentBuilder;
}

impl PreonComponentBuilderStaticTextureExtension for PreonComponentBuilder {
    fn start_static_texture(&mut self, index: usize) -> &mut PreonComponentBuilder {
        self.stack.push(PreonComponent {
            style: PreonStyle {
                background: PreonBackground::Image(PreonImage::from_static(index)),
                ..Default::default()
            },
            ..Default::default()
        });

        self
    }
}