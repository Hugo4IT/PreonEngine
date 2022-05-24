use std::{pin::Pin, task::{Context, Poll}};

use futures_util::{Stream, task::AtomicWaker};

use self::color::Color;
use crate::transform::Rect;

pub mod color;

#[derive(Debug)]
pub struct Canvas {
    instructions: Vec<CanvasInstruction>,
    waker: AtomicWaker,
}

impl Canvas {
    pub fn new() -> Canvas {
        Canvas {
            instructions: Vec::new(),
            waker: AtomicWaker::new(),
        }
    }
}

impl Default for Canvas {
    fn default() -> Self {
        Self::new()
    }
}

impl Canvas {
    pub fn draw_rect(&mut self, rect: Rect, color: Color, thickness: f32) {
        self.instructions
            .push(CanvasInstruction::DrawRect(rect, color, thickness));
    }

    pub fn draw_rounded_rect(&mut self, rect: Rect, color: Color, thickness: f32, radius: f32) {
        self.instructions
            .push(CanvasInstruction::DrawRoundedRect(
                rect, color, thickness, radius,
            ));
    }

    pub fn fill_rect(&mut self, rect: Rect, color: Color) {
        self.instructions
            .push(CanvasInstruction::FillRect(rect, color));
    }

    pub fn fill_rounded_rect(&mut self, rect: Rect, color: Color, radius: f32) {
        self.instructions
            .push(CanvasInstruction::FillRoundedRect(
                rect, color, radius,
            ));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum CanvasInstruction {
    DrawRect(Rect, Color, f32),
    DrawRoundedRect(Rect, Color, f32, f32),
    FillRect(Rect, Color),
    FillRoundedRect(Rect, Color, f32),
}

impl Stream for Canvas {
    type Item = CanvasInstruction;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.as_mut().waker.register(cx.waker());
        match self.as_mut().instructions.pop() {
            Some(i) => Poll::Ready(Some(i)),
            None => Poll::Pending,
        }
    }
}