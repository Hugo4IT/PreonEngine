use components::{PreonComponent, PreonComponentStack, PreonDefaultComponents};
use events::{PreonEvent, PreonEventEmitter};
use rendering::PreonRenderPass;
use types::PreonBox;

use self::types::PreonVector;

/// All default components.
pub mod components;

/// Traits and enums to make your own renderer
pub mod rendering;

/// Mini event system.
pub mod events;

/// Currently only contains `Vector2<T>`.
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

pub trait PreonRenderer {
    fn start(&mut self);
    fn update(&mut self, events: &mut PreonEventEmitter) -> bool;
    fn render(&mut self, render_pass: &mut PreonRenderPass);
}

pub struct PreonEngine<T: PreonComponentStack> {
    pub root: PreonComponent<T>,
    pub model: PreonBox,
    pub events: PreonEventEmitter,
    pub window_inner_size: PreonVector<i32>,
    pub render_pass: PreonRenderPass,
}

impl<T: PreonComponentStack> PreonEngine<T> {
    pub fn new() -> Self {
        Self {
            root: PreonComponent::new(
                T::get_default(PreonDefaultComponents::VBoxComponent)
            ),
            model: PreonBox::initial(),
            events: PreonEventEmitter::new(),
            window_inner_size: PreonVector::zero(),
            render_pass: PreonRenderPass::new(),
        }
    }

    pub fn update(&mut self) {
        // self.root.update(Vector2::zero(), self.window_inner_size);
        // let root_layout = self.root.layout;

        // self.window_inner_size = root_layout.min_size;

        self.events.flip();
        self.render_pass.flip();
    }

    pub fn resize(&mut self, new_size: PreonVector<i32>) {
        self.events.push(PreonEvent::WindowResized { new_size });
    }
}
