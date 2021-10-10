use crate::{
    pipeline::{PreonRenderPipeline, PreonShape},
    types::Vector2,layout::{self, PreonLayout},
    size, utils, PreonComponent,
};

pub struct PreonRect {
    pub position: Vector2<i32>,
    pub size: Vector2<i32>,
    pub color: (f32, f32, f32, f32),
}

impl PreonShape for PreonRect {}

pub struct PreonVertical {
    layout: PreonLayout,
    children: Vec<Box<dyn PreonComponent>>,
    expanding_children: u32,
    id: u32,
}

impl PreonVertical {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            layout: PreonLayout {
                margin: layout::margin(0),
                padding: layout::padding(0),
                min_size: utils::vector2(0),
                size_flags: size::FIT,
            },
            children: Vec::new(),
            expanding_children: 0,
            id: 0,
        })
    }
}

impl PreonComponent for PreonVertical {
    fn add_child(&mut self, new_child: Box<dyn PreonComponent>) {
        self.children.push(new_child);
    }

    fn update(&mut self, position: Vector2<i32>, size: Vector2<i32>) {
        self.layout.min_size = utils::vector2(0);

        let mut current_y: i32 = 0;
        for child in self.children.iter_mut() {
            let child_layout = child.get_layout();

            if self.layout.has_size_flag(size::vertical::FIT) {
                if child_layout.min_size.y
                    + child_layout.margin.top
                    + child_layout.margin.bottom > self.layout.min_size.y
                {
                    self.layout.min_size.y = child_layout.min_size.y
                        + child_layout.margin.top
                        + child_layout.margin.bottom;
                }
            }
            if self.layout.has_size_flag(size::horizontal::FIT) {
                if child_layout.min_size.x
                    + child_layout.margin.left
                    + child_layout.margin.right > self.layout.min_size.x
                {
                    self.layout.min_size.x = child_layout.min_size.x
                        + child_layout.margin.left
                        + child_layout.margin.right;
                }
            }

            current_y += child_layout.min_size.y
            + child_layout.margin.top
            + child_layout.margin.bottom;
        }

        if self.layout.min_size.x < size.x {
            self.layout.min_size.x = size.x;
        }
        
        if self.layout.min_size.y < size.y {
            self.layout.min_size.y = size.y;
        }


        for child in self.children.iter_mut() {
            let child_layout = child.get_layout();

            child.update(
                Vector2 {
                    x: position.x + child_layout.margin.left,
                    y: current_y + child_layout.margin.top,
                },
                Vector2 {
                    x: size.x - child_layout.margin.left - child_layout.margin.right,
                    y: child_layout.min_size.y
                        - child_layout.margin.top
                        - child_layout.margin.bottom,
                },
            );
        }
    }

    fn get_layout(&self) -> PreonLayout {
        self.layout
    }

    fn set_id(&mut self, new_id: u32) {
        self.id = new_id;
    }

    #[inline(always)]
    fn get_id(&self) -> u32 {
        self.id
    }

    fn get_pipeline(&self) -> Option<PreonRenderPipeline> {
        None
    }
}

pub struct PreonButton {
    layout: PreonLayout,
    position: Vector2<i32>,
    size: Vector2<i32>,
    id: u32,
}

impl PreonButton {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            layout: PreonLayout {
                margin: layout::margin(8),
                padding: layout::padding_xy(16, 8),
                min_size: utils::vector2(24),
                size_flags: size::vertical::FIT + size::horizontal::FIT_EXPAND,
            },
            position: utils::vector2(0),
            size: utils::vector2(0),
            id: 0,
        })
    }
}

impl PreonComponent for PreonButton {
    fn add_child(&mut self, _: Box<dyn PreonComponent>) {
        panic!("PreonButton is not made to hold children!");
    }

    fn update(&mut self, position: Vector2<i32>, size: Vector2<i32>) {
        
    }

    fn get_layout(&self) -> PreonLayout {
        self.layout
    }

    fn set_id(&mut self, new_id: u32) {
        self.id = new_id;
    }

    #[inline(always)]
    fn get_id(&self) -> u32 {
        self.id
    }

    fn get_pipeline(&self) -> Option<PreonRenderPipeline> {
        Some(PreonRenderPipeline {
            drawables: Some(vec![Box::new(PreonRect {
                position: self.position,
                size: self.size,
                color: utils::color(0xda0037ff),
            })]),
            children: None,
        })
    }
}
