use self::color::Color;
use crate::transform::Rect;

pub mod color;

pub trait Canvas<'a> {
    fn draw_rect(&mut self, rect: Rect, color: Color, thickness: f32);
    fn draw_rounded_rect(&mut self, rect: Rect, color: Color, thickness: f32, radius: f32);
    fn fill_rect(&mut self, rect: Rect, color: Color);
    fn fill_rounded_rect(&mut self, rect: Rect, color: Color, radius: f32);
    fn get_parent(&'a mut self) -> Option<&'a mut dyn HostCanvas<'a>>;
}

pub trait HostCanvas<'a>: std::fmt::Debug + Canvas<'a> {
    fn derive(&'a mut self) -> &'a mut dyn Canvas;
}

#[derive(Debug)]
pub struct BufferedCanvas<'a> {
    instructions: Vec<BufferedCanvasInstruction>,
    parent: Option<&'a mut dyn HostCanvas<'a>>,
}

impl<'a> BufferedCanvas<'a> {
    pub(crate) fn new() -> BufferedCanvas<'a> {
        BufferedCanvas {
            instructions: Vec::new(),
            parent: None,
        }
    }

    pub(crate) fn from_parent(parent: &'a mut dyn HostCanvas<'a>) -> BufferedCanvas<'a> {
        BufferedCanvas {
            parent: Some(parent),
            ..BufferedCanvas::new()
        }
    }
}

impl<'a> Canvas<'a> for BufferedCanvas<'a> {
    fn draw_rect(&mut self, rect: Rect, color: Color, thickness: f32) {
        self.instructions.push(BufferedCanvasInstruction::DrawRect(rect, color, thickness));
    }

    fn draw_rounded_rect(&mut self, rect: Rect, color: Color, thickness: f32, radius: f32) {
        self.instructions.push(BufferedCanvasInstruction::DrawRoundedRect(rect, color, thickness, radius));
    }

    fn fill_rect(&mut self, rect: Rect, color: Color) {
        self.instructions.push(BufferedCanvasInstruction::FillRect(rect, color));
    }

    fn fill_rounded_rect(&mut self, rect: Rect, color: Color, radius: f32) {
        self.instructions.push(BufferedCanvasInstruction::FillRoundedRect(rect, color, radius));
    }

    fn get_parent(&mut self) -> Option<&'a mut dyn HostCanvas<'a>> {
        self.parent
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum BufferedCanvasInstruction {
    DrawRect(Rect, Color, f32),
    DrawRoundedRect(Rect, Color, f32, f32),
    FillRect(Rect, Color),
    FillRoundedRect(Rect, Color, f32),
}
