use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

use crate::size;

pub trait Vector2Able:
    Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + PartialOrd
    + Copy
    + Clone
    + Display
    + From<u8>
{
}

impl Vector2Able for i16 {}
impl Vector2Able for i32 {}
impl Vector2Able for i64 {}
impl Vector2Able for i128 {}
impl Vector2Able for u8 {}
impl Vector2Able for u16 {}
impl Vector2Able for u32 {}
impl Vector2Able for u64 {}
impl Vector2Able for u128 {}
impl Vector2Able for f32 {}
impl Vector2Able for f64 {}

// pub trait Normalizable {
//     fn normalized(&self) -> Self;
// }

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct PreonVector<T: Vector2Able> {
    pub x: T,
    pub y: T,
}

impl<T: Vector2Able> PreonVector<T> {
    pub fn new(x: T, y: T) -> PreonVector<T> {
        PreonVector { x, y }
    }

    pub fn zero() -> PreonVector<T> {
        PreonVector {
            x: T::from(0u8),
            y: T::from(0u8),
        }
    }

    pub fn one() -> PreonVector<T> {
        PreonVector {
            x: T::from(1u8),
            y: T::from(1u8),
        }
    }
}

impl PreonVector<f32> {
    pub fn normalized(&self) -> PreonVector<f32> {
        if self.x > self.y {
            PreonVector {
                x: 1f32,
                y: self.y / self.x,
            }
        } else {
            PreonVector {
                x: self.x / self.y,
                y: 1f32,
            }
        }
    }

    pub fn normalize(&mut self) {
        if self.x > self.y {
            self.y = self.y / self.x;
            self.x = 1f32;
        } else {
            self.x = self.x / self.y;
            self.y = 1f32;
        }
    }
}

impl PreonVector<f64> {
    pub fn normalized(&self) -> PreonVector<f64> {
        if self.x > self.y {
            PreonVector {
                x: 1f64,
                y: self.y / self.x,
            }
        } else {
            PreonVector {
                x: self.x / self.y,
                y: 1f64,
            }
        }
    }

    pub fn normalize(&mut self) {
        if self.x > self.y {
            self.y = self.y / self.x;
            self.x = 1f64;
        } else {
            self.x = self.x / self.y;
            self.y = 1f64;
        }
    }
}

impl<T: Vector2Able> Add for PreonVector<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Vector2Able> Sub for PreonVector<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: Vector2Able> Mul for PreonVector<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl<T: Vector2Able> Mul<T> for PreonVector<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T: Vector2Able> Div for PreonVector<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl<T: Vector2Able> Div<T> for PreonVector<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T: Vector2Able> Display for PreonVector<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x: {}, y: {})", self.x, self.y)
    }
}

pub struct PreonColor {
    pub r8: u8,
    pub g8: u8,
    pub b8: u8,
    pub a8: u8,
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl PreonColor {
    pub fn from_rgba(r: f32, g: f32, b: f32, a: f32) -> PreonColor {
        PreonColor {
            r8: (r * 255.0f32) as u8,
            g8: (g * 255.0f32) as u8,
            b8: (b * 255.0f32) as u8,
            a8: (a * 255.0f32) as u8,
            r,
            g,
            b,
            a,
        }
    }

    pub fn from_rgba8(r: u8, g: u8, b: u8, a: u8) -> PreonColor {
        PreonColor {
            r8: r,
            g8: g,
            b8: b,
            a8: a,
            r: r as f32 / 255.0f32,
            g: g as f32 / 255.0f32,
            b: b as f32 / 255.0f32,
            a: a as f32 / 255.0f32,
        }
    }

    pub fn from_hex(hex: &'static str) -> PreonColor {
        let cleaned = hex.replace("#", "").replace("0x", "");
        if cleaned.len() <= 4usize {
            PreonColor::from_rgba8(
                u8::from_str_radix(&cleaned[0..0], 16).unwrap(),
                u8::from_str_radix(&cleaned[1..1], 16).unwrap(),
                u8::from_str_radix(&cleaned[2..2], 16).unwrap(),
                (|c: &str, l| -> u8 {
                    if l == 4 {
                        u8::from_str_radix(&c[3..3], 16).unwrap()
                    } else {
                        255
                    }
                })(cleaned.as_str(), cleaned.len()),
            )
        } else if cleaned.len() == 6 || cleaned.len() == 8 {
            PreonColor::from_rgba8(
                u8::from_str_radix(&cleaned[0..1], 16).unwrap(),
                u8::from_str_radix(&cleaned[2..3], 16).unwrap(),
                u8::from_str_radix(&cleaned[4..5], 16).unwrap(),
                (|c: &str, l| -> u8 {
                    if l == 8 {
                        u8::from_str_radix(&c[6..7], 16).unwrap()
                    } else {
                        255
                    }
                })(cleaned.as_str(), cleaned.len()),
            )
        } else {
            PreonColor::from_rgba8(255, 0, 0, 255);
            panic!("Please only use PreonColor::from_hex() with a hex string of 3, 4, 6 or 8 characters long (excluding # or 0x)");
        }
    }

    pub fn into_f32_tuple(&self) -> (f32, f32, f32, f32) {
        (self.r, self.g, self.b, self.a)
    }

    pub fn into_f64_tuple(&self) -> (f64, f64, f64, f64) {
        (self.r as f64, self.g as f64, self.b as f64, self.a as f64)
    }
}

impl Display for PreonColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "R: {}, G: {}, B: {}, A: {}
        R8: {}, G8: {}, B8: {}, A8: {}",
            self.r, self.g, self.b, self.a, self.r8, self.g8, self.b8, self.a8
        )
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct PreonBorder {
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
    pub left: i32,
}

impl PreonBorder {
    pub fn new(top: i32, right: i32, bottom: i32, left: i32) -> PreonBorder {
        PreonBorder {
            top,
            right,
            bottom,
            left,
        }
    }

    pub fn from_xy(x: i32, y: i32) -> PreonBorder {
        PreonBorder::new(y, x, y, x)
    }

    pub fn from_single(value: i32) -> PreonBorder {
        PreonBorder::new(value, value, value, value)
    }

    pub fn zero() -> PreonBorder {
        PreonBorder::from_single(0)
    }

    pub fn x(&self) -> i32 {
        self.left + self.right
    }

    pub fn y(&self) -> i32 {
        self.top + self.bottom
    }
}

impl Add<PreonBorder> for PreonVector<i32> {
    type Output = PreonVector<i32>;

    fn add(self, rhs: PreonBorder) -> Self::Output {
        PreonVector::new(rhs.x() + self.x, rhs.y() + self.y)
    }
}

impl Sub<PreonBorder> for PreonVector<i32> {
    type Output = PreonVector<i32>;

    fn sub(self, rhs: PreonBorder) -> Self::Output {
        PreonVector::new(rhs.x() - self.x, rhs.y() - self.y)
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct PreonCorners {
    pub top_left: i32,
    pub top_right: i32,
    pub bottom_right: i32,
    pub bottom_left: i32,
}

impl PreonCorners {
    pub fn new(top_left: i32, top_right: i32, bottom_right: i32, bottom_left: i32) -> PreonCorners {
        PreonCorners {
            top_left,
            top_right,
            bottom_right,
            bottom_left,
        }
    }

    pub fn from_xy(x: i32, y: i32) -> PreonCorners {
        PreonCorners::new(y, x, y, x)
    }

    pub fn from_single(value: i32) -> PreonCorners {
        PreonCorners::new(value, value, value, value)
    }

    pub fn pill(rect: PreonVector<i32>) -> PreonCorners {
        if rect.x > rect.y {
            PreonCorners::from_single(rect.y / 2)
        } else {
            PreonCorners::from_single(rect.x / 2)
        }
    }

    pub fn zero() -> PreonCorners {
        PreonCorners::from_single(0)
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct PreonBox {
    pub margin: PreonBorder,
    pub padding: PreonBorder,
    pub border: PreonBorder,
    pub size_flags: u8,
    pub min_size: PreonVector<i32>,
}

impl PreonBox {
    pub fn initial() -> PreonBox {
        PreonBox {
            margin: PreonBorder::zero(),
            padding: PreonBorder::zero(),
            border: PreonBorder::zero(),
            size_flags: size::EXPAND,
            min_size: PreonVector::zero(),
        }
    }

    pub fn has_flag(&self, flag: u8) -> bool {
        (self.size_flags & flag) == flag
    }
}
