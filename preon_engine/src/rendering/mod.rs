use alloc::vec::Vec;

use crate::types::{Color, Corners, Position, Size};

pub struct RenderPass {
    buffer: Vec<Shape>,
    pass: Vec<Shape>,
}

impl RenderPass {
    pub fn new() -> RenderPass {
        RenderPass {
            buffer: Vec::new(),
            pass: Vec::new(),
        }
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.pass.is_empty()
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.pass.len()
    }

    pub fn push(&mut self, item: Shape) {
        self.buffer.push(item);
    }

    pub fn flip(&mut self) {
        self.pass.truncate(self.buffer.len());
        self.pass.clone_from_slice(&self.buffer[..]);
    }
}

#[derive(Debug, Clone)]
pub enum Shape {
    Rect(Position, Size, Color),
    RoundedRect(Position, Size, Color, Corners),
}
