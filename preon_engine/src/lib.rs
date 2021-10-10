use std::{cell::RefCell, rc::Rc};

use events::{PreonEvent, PreonEventEmitter, WindowEventArgs};
use pipeline::PreonRenderPipeline;

use self::{
    components::PreonVertical,
    layout::PreonLayout,
    types::Vector2,
};

/// All default components.
pub mod components;

/// Mini event system.
pub mod events;

/// Datatypes used for computing layout.
pub mod layout;

/// Currently only contains `Vector2<T>`.
pub mod types;

/// Tiny utility functions for cleaner or more consistent syntax.
pub mod utils;

/// Specify how to render this component
pub mod pipeline;

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
    fn update(&mut self, engine: &mut PreonEngine) -> bool;
    fn render(&mut self, engine: &mut PreonEngine);
}

pub trait PreonComponent {
    fn add_child(&mut self, new_child: Box<dyn PreonComponent>);
    fn update(&mut self, position: Vector2<i32>, size: Vector2<i32>);
    fn get_pipeline(&self) -> Option<PreonRenderPipeline>;
    fn get_layout(&self) -> PreonLayout;
    fn set_id(&mut self, new_id: u32);
    fn get_id(&self) -> u32;
}

// Used by PreonRenderers to make their own trait
pub trait PreonRenderableComponent<T: PreonRenderer> {}

pub struct PreonEngine {
    pub root: Box<PreonVertical>,
    pub layout: PreonLayout,

    pub events: PreonEventEmitter,

    pub window_inner_size: Vector2<i32>,
    _window_inner_size: Vector2<i32>,
}

impl PreonEngine {
    pub fn new() -> Self {
        Self {
            root: PreonVertical::new(),
            layout: PreonLayout {
                margin: layout::margin(0),
                padding: layout::padding(0),
                min_size: utils::vector2(0),
                size_flags: size::FIT,
            },
            events: PreonEventEmitter::new(),
            window_inner_size: utils::vector2(0),
            _window_inner_size: utils::vector2(0),
        }
    }

    pub fn update(&mut self) {
        self.root.update(utils::vector2(0), self.window_inner_size);
        let root_layout = self.root.get_layout();

        self.window_inner_size = root_layout.min_size;
        if self._window_inner_size != self.window_inner_size {
            self.resize(self.window_inner_size);
        }

        self.events.flip();
    }

    pub fn resize(&mut self, new_size: Vector2<i32>) {
        self._window_inner_size = new_size;
        self.events.push(
            PreonEvent::Window(
                WindowEventArgs::Resized { new_size }
            )
        );
    }
}