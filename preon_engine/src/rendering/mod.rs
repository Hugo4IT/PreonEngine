use crate::{events::PreonEventEmitter, types::{PreonColor, PreonVector}};

pub trait PreonRenderer {
    fn start(&mut self);
    fn update(&mut self, events: &mut PreonEventEmitter);
    fn render(&mut self, render_pass: &mut PreonRenderPass);
}

#[derive(Debug, Copy, Clone)]
pub enum PreonShape {
    Rect {
        position: PreonVector<i32>,
        size: PreonVector<i32>,
        color: PreonColor,
    }
}

#[derive(Debug)]
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

    pub fn pull<F: FnMut(PreonShape)>(&mut self, mut handler: F) {
        let mut pass = self.pass.iter();
        while let Some(item) = pass.next() {
            handler(*item);
        }
    }

    pub fn flip(&mut self) {
        self.pass = self.buffer.drain(..).collect();
    }

    #[inline(always)]
    pub fn len(&self) -> usize { self.pass.len() }
}
