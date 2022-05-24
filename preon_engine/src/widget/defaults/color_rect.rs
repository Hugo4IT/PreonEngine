use crate::{widget::Widget, canvas::{color::Color, Canvas}};

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

impl Widget for ColorRect {
    fn view(&mut self, canvas: &mut Canvas) {
        canvas.fill_rect(((0, 0), (200, 200)).into(), self.color);
    }
}