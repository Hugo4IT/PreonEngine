use core::ops::{Add, Sub, Mul, Div};

use self::vector::{Vec2, Vec4};

pub mod vector;

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn clip(self, bounds: Size) -> Position {
        Position {
            x: self.x.min(bounds.width),
            y: self.y.min(bounds.height),
        }
    }
}

impl<X, Y> From<(X, Y)> for Position
where
    f64: core::convert::From<X>,
    f64: core::convert::From<Y>,
{
    fn from((x, y): (X, Y)) -> Position {
        Position {
            x: f64::from(x) as f32,
            y: f64::from(y) as f32,
        }
    }
}

impl From<Position> for Vec2 {
    fn from(position: Position) -> Vec2 {
        Vec2(position.x, position.y)
    }
}

impl<T: Into<Vec2>> Add<T> for Position {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        let Vec2(x, y) = rhs.into();
        Position {
            x: self.x + x,
            y: self.y + y,
        }
    }
}

impl<T: Into<Vec2>> Sub<T> for Position {
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        let Vec2(x, y) = rhs.into();
        Position {
            x: self.x - x,
            y: self.y - y,
        }
    }
}

impl<T: Into<Vec2>> Mul<T> for Position {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let Vec2(x, y) = rhs.into();
        Position {
            x: self.x * x,
            y: self.y * y,
        }
    }
}

impl<T: Into<Vec2>> Div<T> for Position {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        let Vec2(x, y) = rhs.into();
        Position {
            x: self.x / x,
            y: self.y / y,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub fn clip(self, bounds: Size) -> Size {
        Size {
            width: self.width.min(bounds.width),
            height: self.height.min(bounds.height),
        }
    }
}

impl From<Size> for Vec2 {
    fn from(size: Size) -> Vec2 {
        Vec2(size.width, size.height)
    }
}

impl<Width, Height> From<(Width, Height)> for Size
where
    f64: core::convert::From<Width>,
    f64: core::convert::From<Height>,
{
    fn from((width, height): (Width, Height)) -> Size {
        Size {
            width: f64::from(width) as f32,
            height: f64::from(height) as f32,
        }
    }
}

impl<T: Into<Vec2>> Add<T> for Size {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        let Vec2(width, height) = rhs.into();
        Size {
            width: self.width + width,
            height: self.height + height,
        }
    }
}

impl<T: Into<Vec2>> Sub<T> for Size {
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        let Vec2(width, height) = rhs.into();
        Size {
            width: self.width - width,
            height: self.height - height,
        }
    }
}

impl<T: Into<Vec2>> Mul<T> for Size {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let Vec2(width, height) = rhs.into();
        Size {
            width: self.width * width,
            height: self.height * height,
        }
    }
}

impl<T: Into<Vec2>> Div<T> for Size {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        let Vec2(width, height) = rhs.into();
        Size {
            width: self.width / width,
            height: self.height / height,
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

impl From<Rect> for Vec4 {
    fn from(rect: Rect) -> Vec4 {
        Vec4(rect.0.x, rect.0.y, rect.1.width, rect.1.height)
    }
}

impl<P: Into<Position>, S: Into<Size>> From<(P, S)> for Rect {
    fn from((position, size): (P, S)) -> Self {
        Rect(position.into(), size.into())
    }
}