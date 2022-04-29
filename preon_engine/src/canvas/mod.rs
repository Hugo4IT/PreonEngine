use self::color::Color;
use crate::transform::Rect;

pub mod color;

pub trait Canvas {
    type Instruction;

    fn draw_rect(&mut self, rect: Rect, color: Color, thickness: f32);
    fn draw_rounded_rect(&mut self, rect: Rect, color: Color, thickness: f32, radius: f32);
    fn fill_rect(&mut self, rect: Rect, color: Color);
    fn fill_rounded_rect(&mut self, rect: Rect, color: Color, radius: f32);

    fn end(self) -> Vec<Self::Instruction>;
}

#[derive(Debug)]
pub struct SubCanvas<'a, T: Canvas> {
    parent: &'a mut T,
    region: Rect,
}

impl<'a, T: Canvas> SubCanvas<'a, T> {
    #[inline]
    pub(crate) fn new(parent: &'a mut T, region: Rect) -> SubCanvas<'a, T> {
        SubCanvas { parent, region }
    }

    #[inline]
    pub(crate) fn transform_rect(&self, rect: Rect) -> Rect {
        Rect(
            (rect.get_position() + self.region.get_position()).clip(self.region.get_size()),
            (rect.get_size() + rect.get_position())
                .clip(self.region.get_size() - rect.get_position()),
        )
    }
}

impl<'a, T: Canvas> Canvas for SubCanvas<'a, T> {
    type Instruction = T::Instruction;

    fn draw_rect(&mut self, rect: Rect, color: Color, thickness: f32) {
        self.parent
            .draw_rect(self.transform_rect(rect), color, thickness)
    }

    fn draw_rounded_rect(&mut self, rect: Rect, color: Color, thickness: f32, radius: f32) {
        self.parent
            .draw_rounded_rect(self.transform_rect(rect), color, thickness, radius)
    }

    fn fill_rect(&mut self, rect: Rect, color: Color) {
        self.parent.fill_rect(self.transform_rect(rect), color)
    }

    fn fill_rounded_rect(&mut self, rect: Rect, color: Color, radius: f32) {
        self.parent
            .fill_rounded_rect(self.transform_rect(rect), color, radius)
    }

    fn end(self) -> Vec<T::Instruction> {
        Vec::new()
    }
}

#[derive(Debug)]
pub struct BufferedCanvas {
    instructions: Vec<BufferedCanvasInstruction>,
}

impl BufferedCanvas {
    pub fn new() -> BufferedCanvas {
        BufferedCanvas {
            instructions: Vec::new(),
        }
    }

    pub fn derive(&mut self, region: Rect) -> SubCanvas<BufferedCanvas> {
        SubCanvas::new(self, region)
    }
}

impl Default for BufferedCanvas {
    fn default() -> Self {
        Self::new()
    }
}

impl Canvas for BufferedCanvas {
    type Instruction = BufferedCanvasInstruction;

    fn draw_rect(&mut self, rect: Rect, color: Color, thickness: f32) {
        self.instructions
            .push(BufferedCanvasInstruction::DrawRect(rect, color, thickness));
    }

    fn draw_rounded_rect(&mut self, rect: Rect, color: Color, thickness: f32, radius: f32) {
        self.instructions
            .push(BufferedCanvasInstruction::DrawRoundedRect(
                rect, color, thickness, radius,
            ));
    }

    fn fill_rect(&mut self, rect: Rect, color: Color) {
        self.instructions
            .push(BufferedCanvasInstruction::FillRect(rect, color));
    }

    fn fill_rounded_rect(&mut self, rect: Rect, color: Color, radius: f32) {
        self.instructions
            .push(BufferedCanvasInstruction::FillRoundedRect(
                rect, color, radius,
            ));
    }

    fn end(self) -> Vec<Self::Instruction> {
        self.instructions
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum BufferedCanvasInstruction {
    DrawRect(Rect, Color, f32),
    DrawRoundedRect(Rect, Color, f32, f32),
    FillRect(Rect, Color),
    FillRoundedRect(Rect, Color, f32),
}
