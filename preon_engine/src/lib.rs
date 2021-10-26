use components::{PreonComponent, PreonCustomComponentStack};
use events::{PreonEvent, PreonEventEmitter};
use rendering::PreonRenderPass;
use types::PreonBox;

use crate::components::PreonComponentStack;

use self::types::PreonVector;

/// All default components.
pub mod components;

/// Traits and enums to make your own renderer.
pub mod rendering;

/// Mini event system.
pub mod events;

/// Basically default values for components.
pub mod theme;

/// All Preon* utility structs like PreonVector<T>, PreonColor and PreonBox.
pub mod types;

/// Size flags shortcuts.
pub mod size {
    /// Only apply a specific size flag to the X axis.
    pub mod horizontal {
        /// Automatically resize to fit children horizontally.
        pub const FIT: u8 = 0b00000001;

        /// Expand to horizontally fill leftover space in parent.
        pub const EXPAND: u8 = 0b00000010;

        /// Resize to fit children, but expand to available space.
        pub const FIT_EXPAND: u8 = FIT + EXPAND;
    }

    /// Only apply a specific size flag to the Y axis.
    pub mod vertical {
        /// Automatically resize to fit children vertically.
        pub const FIT: u8 = 0b00000100;

        /// Expand to vertically fill leftover space in parent.
        pub const EXPAND: u8 = 0b00001000;

        /// Resize to fit children, but expand to available space.
        pub const FIT_EXPAND: u8 = FIT + EXPAND;
    }

    /// Automatically resize to fit children.
    pub const FIT: u8 = horizontal::FIT + vertical::FIT;

    /// Expand to fill leftover space in parent.
    pub const EXPAND: u8 = horizontal::EXPAND + vertical::EXPAND;

    /// Resize to fit children, but expand to available space.
    pub const FIT_EXPAND: u8 = FIT + EXPAND;
}

pub struct PreonEngine<T: PreonCustomComponentStack> {
    pub tree: PreonComponent<T>,
    pub model: PreonBox,
    pub events: PreonEventEmitter,
    pub window_inner_size: PreonVector<i32>,
    pub render_pass: PreonRenderPass,
}

impl<T: PreonCustomComponentStack> PreonEngine<T> {
    pub fn new(tree: PreonComponent<T>) -> Self {
        Self {
            tree,
            model: PreonBox::initial(),
            events: PreonEventEmitter::new(),
            window_inner_size: PreonVector::zero(),
            render_pass: PreonRenderPass::new(),
        }
    }

    pub fn update(&mut self, ) -> bool {
        

        self.events.flip();
        self.render_pass.flip();

        self.events.len() > 0
    }

    fn handle_component<F>(&mut self, component: PreonComponent<T>, custom: &mut F)
    where
        F: FnMut(&mut T, &Option<Vec<PreonComponent<T>>>) + 'static
    {
        if let Some(children) = component.children {
            for child in children.iter() {
                self.handle_component(child, custom);
            }
        }

        match component.data {
            PreonComponentStack::Custom(mut c) => custom(&mut c, &component.children),
            PreonComponentStack::RectComponent { color } => todo!(),
            PreonComponentStack::VBoxComponent => todo!(),
        }
    }

    pub fn resize(&mut self, new_size: PreonVector<i32>) {
        self.events.push(PreonEvent::WindowResized { new_size });
    }
}
