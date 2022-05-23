use crate::{widget::Widget, canvas::{color::Color, Canvas, SubCanvas}};

pub struct ColorRect {
    color: Color,
}

impl ColorRect {
    pub fn new(color: Color) -> ColorRect {
        ColorRect {
            color,
        }
    }
}

impl<C: Canvas> Widget<C> for ColorRect {
    fn view(&mut self, mut canvas: SubCanvas<C>) {
        canvas.fill_rect(((0, 0), (200, 200)).into(), self.color);
    }
}