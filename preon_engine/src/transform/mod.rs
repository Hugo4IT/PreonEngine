use self::vector::{Vec2, Vec4};

pub mod vector;

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl<X, Y> From<(X, Y)> for Position
where
    X: Into<f32>,
    Y: Into<f32>,
{
    fn from((x, y): (X, Y)) -> Position {
        Position {
            x: x.into(),
            y: y.into(),
        }
    }
}

impl Into<Vec2> for Position {
    fn into(self) -> Vec2 {
        Vec2(self.x, self.y)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Into<Vec2> for Size {
    fn into(self) -> Vec2 {
        Vec2(self.width, self.height)
    }
}

impl<Width, Height> From<(Width, Height)> for Size
where
    Width: Into<f32>,
    Height: Into<f32>,
{
    fn from((width, height): (Width, Height)) -> Size {
        Size {
            width: width.into(),
            height: height.into(),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Rect(pub Position, pub Size);

impl Rect {
    #[inline(always)]
    pub fn get_position(&self) -> Position {
        self.0
    }

    #[inline(always)]
    pub fn get_size(&self) -> Size {
        self.1
    }
}

impl Into<Vec4> for Rect {
    fn into(self) -> Vec4 {
        Vec4(self.0.x, self.0.y, self.1.width, self.1.height)
    }
}
