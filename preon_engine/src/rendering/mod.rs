use crate::types::{PreonColor, PreonVector};

pub enum PreonShape {
    Rect {
        position: PreonVector<i32>,
        size: PreonVector<i32>,
        color: PreonColor,
    }
}

pub struct PreonRenderPass {
    pass: Vec<PreonShape>,
    buffer: Vec<PreonShape>
}

impl PreonRenderPass {
    pub fn new() -> PreonRenderPass {
        PreonRenderPass {
            pass: Vec::new(),
            buffer: Vec::new(),
        }
    }

    pub fn push(&mut self, shape: PreonShape) {
        self.buffer.push(shape);
    }

    pub fn pull<F: FnMut(&PreonShape)>(&mut self, mut handler: F) {
        let mut pass = self.pass.iter();
        while let Some(item) = pass.next() {
            handler(item);
        }
    }

    pub fn flip(&mut self) {
        self.pass.clear();
        for item in self.buffer.drain(..) {
            self.pass.push(item);
        }
    }
}
