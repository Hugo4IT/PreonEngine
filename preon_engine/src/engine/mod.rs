use self::{components::PreonVertical, events::PreonEvent, layout::PreonLayout, types::Vector2};

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

/// Size flags shortcuts.
pub mod size {
    /// Only apply a specific size flag to the X axis.
    pub mod horizontal {
        /// Automatically resize to fit children horizontally.
        pub const FIT: u8 = 0b00000001;

        /// Expand to horizontally fill leftover space in parent.
        pub const EXPAND: u8 = 0b00000010;

        /// Resize to fit children, but expand to available space.
        pub const FILL_EXPAND: u8 = FIT + EXPAND;
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

pub trait PreonRenderer<'a> {
    fn connect_to(&'a mut self, engine: &'a mut PreonEngine);
    fn start(&mut self);
    fn update(&mut self) -> bool;
    fn render(&mut self);
}

pub trait PreonComponent {
    fn add_child(&mut self, new_child: Box<dyn PreonComponent>);
    fn layout(&mut self, parent: PreonLayout) -> PreonLayout;
}

// Used by PreonRenderers to make their own trait
pub trait PreonRenderableComponent<'a, T: PreonRenderer<'a>> {}

pub struct PreonEngine {
    pub root: Box<PreonVertical>,
    pub layout: PreonLayout,

    pub on_resized: PreonEvent<Vector2<u32>>,

    window_inner_size: Vector2<u32>,
    _window_inner_size: Vector2<u32>,
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
            on_resized: PreonEvent::new(),
            window_inner_size: utils::vector2(0),
            _window_inner_size: utils::vector2(0),
        }
    }

    pub fn update(&mut self) {
        let root_layout = self.root.layout(self.layout);

        self.window_inner_size = root_layout.get_min_size();
        if self._window_inner_size != self.window_inner_size {
            self.resize(self.window_inner_size);
        }
    }

    pub fn resize(&mut self, new_size: Vector2<u32>) {
        self._window_inner_size = new_size;
        println!("It doesn't work, does it");
        self.on_resized.emit(new_size);
    }
}