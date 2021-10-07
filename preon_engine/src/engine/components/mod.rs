use super::{
    layout::{self, PreonLayout},
    size,
    utils, PreonComponent,
};

pub struct PreonRect {
    pub layout: PreonLayout,
    pub color: (f32, f32, f32, f32),
}

impl PreonRect {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            layout: PreonLayout {
                margin: layout::margin(0),
                padding: layout::padding(0),
                min_size: utils::vector2(0),
                size_flags: size::FILL_EXPAND,
            },
            color: utils::color(0xda0037ff),
        })
    }
}

impl PreonComponent for PreonRect {
    fn add_child(&mut self, _new_child: Box<dyn PreonComponent>) {
        panic!("PreonRect is not made to hold children!")
    }

    fn layout(&mut self, _parent: PreonLayout) -> PreonLayout {
        self.layout
    }
}

pub struct PreonVertical {
    pub layout: PreonLayout,
    pub children: Vec<Box<dyn PreonComponent>>,
    pub expanding_children: u32,
}

impl PreonVertical {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            layout: PreonLayout {
                margin: layout::margin(0),
                padding: layout::padding(0),
                min_size: utils::vector2(0),
                size_flags: size::FILL,
            },
            children: Vec::new(),
            expanding_children: 0,
        })
    }
}

impl PreonComponent for PreonVertical {
    fn add_child(&mut self, new_child: Box<dyn PreonComponent>) {
        self.children.push(new_child);
    }

    fn layout(&mut self, _parent: PreonLayout) -> PreonLayout {
        self.layout.min_size = utils::vector2(0);

        for child in self.children.iter_mut() {
            let child_hints = child.layout(self.layout);
            let child_minsize = child_hints.get_min_size();

            if self.layout.has_size_flag(size::vertical::FILL) {
                if child_minsize.y > self.layout.min_size.y {
                    self.layout.min_size.y = child_minsize.y;
                }
            }
            if self.layout.has_size_flag(size::horizontal::FILL) {
                if child_minsize.x > self.layout.min_size.x {
                    self.layout.min_size.x = child_minsize.x;
                }
            }
        }

        self.layout
    }
}
