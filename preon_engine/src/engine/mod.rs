use self::{components::PreonVertical, events::PreonEvent, layout::PreonLayout, types::Vector2};

pub mod components;
pub mod events;
pub mod layout;
pub mod types;
pub mod utils;

pub mod size {
    pub mod horizontal {
        pub const FILL: u8 = 0b00000001;
        pub const EXPAND: u8 = 0b00000010;
        pub const FILL_EXPAND: u8 = FILL + EXPAND;
    }

    pub mod vertical {
        pub const FILL: u8 = 0b00000100;
        pub const EXPAND: u8 = 0b00001000;
        pub const FILL_EXPAND: u8 = FILL + EXPAND;
    }

    pub const EXPAND: u8 = horizontal::EXPAND + vertical::EXPAND;
    pub const FILL: u8 = horizontal::FILL + vertical::FILL;
    pub const FILL_EXPAND: u8 = FILL + EXPAND;
}

pub trait PreonRenderer {
    fn start(&mut self, engine: &PreonEngine);
    fn update(&mut self, engine: &mut PreonEngine) -> bool;
    fn render(&mut self, engine: &PreonEngine);
}

pub trait PreonComponent {
    fn add_child(&mut self, new_child: Box<dyn PreonComponent>);
    fn layout(&mut self, parent: PreonLayout) -> PreonLayout;
}

// Used by PreonRenderers to make their own trait
pub trait PreonRenderableComponent<T: PreonRenderer> {}

pub struct PreonEngine {
    pub root: Box<PreonVertical>,
    pub layout: PreonLayout,

    pub on_resize: PreonEvent<Vector2<u32>>,

    window_inner_size: Vector2<u32>,
    _window_inner_size: Vector2<u32>,
}

impl PreonEngine {
    pub fn init() -> Self {
        Self {
            root: PreonVertical::new(),
            layout: PreonLayout {
                margin: layout::margin(0),
                padding: layout::padding(0),
                min_size: utils::vector2(0),
                size_flags: size::FILL,
            },
            on_resize: PreonEvent::new(),
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
    }
}
