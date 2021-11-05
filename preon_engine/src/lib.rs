use std::any::Any;

use components::{PreonComponent, PreonCustomComponentStack};
use events::{PreonEvent, PreonEventEmitter, PreonUserEvent};
use rendering::PreonRenderPass;
use types::PreonBox;

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
    pub events: PreonEventEmitter<PreonEvent>,
    pub window_inner_size: PreonVector<u32>,
    pub render_pass: PreonRenderPass,
}

impl<T: PreonCustomComponentStack + Any + 'static> PreonEngine<T> {
    pub fn new(tree: PreonComponent<T>) -> Self {
        Self {
            tree,
            model: PreonBox::initial(),
            events: PreonEventEmitter::new(),
            window_inner_size: PreonVector::zero(),
            render_pass: PreonRenderPass::new(),
        }
    }

    pub fn start(&mut self) {}

    pub fn update(&mut self, user_events: &PreonEventEmitter<PreonUserEvent>) -> bool {
        if user_events.len() > 0 || self.events.len() > 0 {
            let mut update_layout = false;

            user_events.pull(|f| match f {
                PreonUserEvent::WindowResized(s) => {
                    self.resize(s);
                    update_layout = true
                }
                PreonUserEvent::ForceLayoutUpdate | PreonUserEvent::WindowOpened => {
                    update_layout = true
                }
                _ => {}
            });

            if update_layout {
                self.tree.set_outer_size(PreonVector::new(
                    self.window_inner_size.x as i32,
                    self.window_inner_size.y as i32,
                ));
                self.tree.set_outer_position(PreonVector::zero());
                T::layout(&mut self.tree);
            }
            T::render(&mut self.tree, &mut self.render_pass);

            self.events.flip();
            self.render_pass.flip();

            true
        } else {
            false
        }
    }

    pub fn resize(&mut self, new_size: PreonVector<u32>) {
        if new_size != self.window_inner_size {
            self.window_inner_size = new_size;
            self.events.push(PreonEvent::WindowResized(new_size));
        }
    }
}
