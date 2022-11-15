use crate::{components::{PreonComponentStorage, PreonComponentStack}, log};

use super::{PreonCustomComponentStack, PreonComponentBuilder};

pub trait AddStaticTexture<T: PreonCustomComponentStack> {
    fn start_static_texture(&mut self, index: usize) -> &mut PreonComponentBuilder<T>;
}

impl<T: PreonCustomComponentStack> AddStaticTexture<T> for PreonComponentBuilder<T> {
    fn start_static_texture(&mut self, index: usize) -> &mut PreonComponentBuilder<T> {
        log::info!("start static texture: {}", index);

        self.stack.push(PreonComponentStorage {
            data: PreonComponentStack::StaticTexture {
                texture_index: index,
            },
            ..Default::default()
        });

        self
    }
}