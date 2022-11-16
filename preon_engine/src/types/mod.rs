use alloc::{format, string::String};

use core::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

use crate::abs;

pub trait PreonVectorAble:
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

impl<T> PreonVectorAble for T
where
    T: Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + PartialOrd
    + Copy
    + Clone
    + Display
    + From<u8>
{}

// A vector (from math, not the array-one) with 2 axis. Useful for storing positions or sizes
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct PreonVector<T: PreonVectorAble> {
    pub x: T,
    pub y: T,
}

impl<T: PreonVectorAble> PreonVector<T> {
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

    /// The dot product of `self` and `rhs` (formula: `self.x * rhs.x + self.y * rhs.y`)
    pub fn dot(&self, rhs: &PreonVector<T>) -> T {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl PreonVector<f32> {
    /// Distance between (0, 0) and (self.x, self.y)
    pub fn length32(&self) -> f32 {
        abs!(libm::sqrtf(self.x * self.x + self.y * self.y))
    }

    /// `self` divided by it's length
    pub fn normalized32(&self) -> PreonVector<f32> {
        if self.x == 0.0 && self.y == 0.0 {
            PreonVector::one()
        } else {
            let length = self.length32();
            PreonVector {
                x: self.x / length,
                y: self.y / length,
            }
        }

    }

    /// `self` divided by it's length
    pub fn normalize32(&mut self) {
        if self.x == 0.0 && self.y == 0.0 {
            self.x = 1.0;
            self.y = 1.0;
        } else {
            let length = self.length32();
            self.x /= length;
            self.y /= length;
        }
    }
}

impl PreonVector<f64> {
    /// Distance between (0, 0) and (self.x, self.y)
    pub fn length(&self) -> f64 {
        abs!(libm::sqrt(self.x * self.x + self.y * self.y))
    }

    /// `self` divided by it's length
    pub fn normalized(&self) -> PreonVector<f64> {
        if self.x == 0.0 && self.y == 0.0 {
            PreonVector::one()
        } else {
            let length = self.length();
            PreonVector {
                x: self.x / length,
                y: self.y / length,
            }
        }

    }

    /// `self` divided by it's length
    pub fn normalize(&mut self) {
        if self.x == 0.0 && self.y == 0.0 {
            self.x = 1.0;
            self.y = 1.0;
        } else {
            let length = self.length();
            self.x /= length;
            self.y /= length;
        }
    }
}

impl<T: PreonVectorAble> Add for PreonVector<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: PreonVectorAble> Sub for PreonVector<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: PreonVectorAble> Mul for PreonVector<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl<T: PreonVectorAble> Mul<T> for PreonVector<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T: PreonVectorAble> Div for PreonVector<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl<T: PreonVectorAble> Div<T> for PreonVector<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T: PreonVectorAble> Display for PreonVector<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(x: {}, y: {})", self.x, self.y)
    }
}

/// Color values with advanced features. Automatically applies SRGB to linear color space conversion.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PreonColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl PreonColor {
    pub const WHITE: PreonColor = PreonColor { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const TRANSPARENT: PreonColor = PreonColor { r: 1.0, g: 1.0, b: 1.0, a: 0.0 };
    pub const BLACK: PreonColor = PreonColor { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const TRANSPARENT_BLACK: PreonColor = PreonColor { r: 0.0, g: 0.0, b: 0.0, a: 0.0 };

    /// Same as `PreonColor::from_rgba(...)`
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> PreonColor {
        PreonColor::from_rgba(r, g, b, a)
    }

    /// Generate a PreonColor value from 4 float values going from 0 to 1, color space conversion is applied.
    pub fn from_rgba(r: f32, g: f32, b: f32, a: f32) -> PreonColor {
        PreonColor {
            r: libm::powf(r, 2.2),
            g: libm::powf(g, 2.2),
            b: libm::powf(b, 2.2),
            a,
        }
    }

    /// Generate a PreonColor value from 4 8-bit unsigned integers ranging from 0 to 255, color values become much easier to read in this format.
    pub fn from_rgba8(r: u8, g: u8, b: u8, a: u8) -> PreonColor {
        PreonColor::from_rgba(
            r as f32 / 255.0f32,
            g as f32 / 255.0f32,
            b as f32 / 255.0f32,
            a as f32 / 255.0f32,
        )
    }

    /// Generate a PreonColor value from a string containing a hex value, similair to CSS.
    ///
    /// ### Supported formats
    ///
    /// Input string|Output color
    /// ---:|:---
    /// `"da003755"`|`#da003755`
    /// `"da0037"`|`#da0037ff`
    /// `"4445"`|`#44444455`
    /// `"444"`|`#444444ff`
    /// `"#da003755"`|`#da003755`
    /// `"#da0037"`|`#da0037ff`
    /// `"#4445"`|`#44444455`
    /// `"#444"`|`#444444ff`
    /// `"0xda003755"`|`#da003755`
    /// `"0xda0037"`|`#da0037ff`
    /// `"0x4445"`|`#44444455`
    /// `"0x444"`|`#444444ff`
    ///
    /// ### Editor Integration
    ///
    /// To get a visual preview of your colors in Visual Studio Code, use the [Colorize](https://marketplace.visualstudio.com/items?itemName=kamikillerto.vscode-colorize) extension, and add [this](https://gist.github.com/Hugo4IT/0defde4eb0bf1c8cac093498c9d474fd) to your `settings.json`
    pub fn from_hex(hex: &str) -> PreonColor {
        let cleaned = hex.replace("#", "").replace("0x", "");
        if cleaned.len() <= 4usize {
            PreonColor::from_rgba8(
                {
                    let c = u8::from_str_radix(&cleaned[0..1], 16).unwrap();
                    if cfg!(target_endian = "little") {
                        c | c << 4
                    } else {
                        c | c >> 4
                    }
                },
                {
                    let c = u8::from_str_radix(&cleaned[1..2], 16).unwrap();
                    if cfg!(target_endian = "little") {
                        c | c << 4
                    } else {
                        c | c >> 4
                    }
                },
                {
                    let c = u8::from_str_radix(&cleaned[2..3], 16).unwrap();
                    if cfg!(target_endian = "little") {
                        c | c << 4
                    } else {
                        c | c >> 4
                    }
                },
                {
                    if cleaned.len() == 4 {
                        let c = u8::from_str_radix(&cleaned[3..4], 16).unwrap();
                        if cfg!(target_endian = "little") {
                            c | c << 4
                        } else {
                            c | c >> 4
                        }
                    } else {
                        255
                    }
                },
            )
        } else if cleaned.len() == 6 || cleaned.len() == 8 {
            PreonColor::from_rgba8(
                u8::from_str_radix(&cleaned[0..2], 16).unwrap(),
                u8::from_str_radix(&cleaned[2..4], 16).unwrap(),
                u8::from_str_radix(&cleaned[4..6], 16).unwrap(),
                {
                    if cleaned.len() == 8 {
                        u8::from_str_radix(&cleaned[6..8], 16).unwrap()
                    } else {
                        255
                    }
                },
            )
        } else {
            PreonColor::from_rgba8(255, 0, 0, 255);
            panic!("Please only use PreonColor::from_hex() with a hex string of 3, 4, 6 or 8 characters long (excluding # or 0x)");
        }
    }

    /// Multiplies the color's values (excluding alpha) by `1.0 + amount`, making them brighter. If you want to keep this PreonColor the same, but also get a lighter version of it, see [`Self::lightened()`]
    pub fn lighten(&mut self, amount: f32) {
        let Self { r, g, b, .. } = self.lightened(amount);
        self.r = r;
        self.g = g;
        self.b = b;
    }

    /// Returns a copy of `self` where all color values (excluding alpha) are multiplied by `1.0 + amount`, making them brighter. If you want to mutate `self` instead of copying, see [`Self::lighten()`]
    pub fn lightened(&self, amount: f32) -> PreonColor {
        let linear = self.into_linear();

        PreonColor::from_rgba(
            linear.r * (1.0 + amount),
            linear.g * (1.0 + amount),
            linear.b * (1.0 + amount),
            self.a,
        )
    }

    /// Multiplies the color's values (excluding alpha) by `1.0 - amount`, making them darker. If you want to keep this PreonColor the same, but also get a darker version of it, see [`Self::darkened()`]
    pub fn darken(&mut self, amount: f32) {
        let Self { r, g, b, .. } = self.darkened(amount);
        self.r = r;
        self.g = g;
        self.b = b;
    }

    /// Returns a copy of `self` where all color values (excluding alpha) are multiplied by `1.0 - amount`, making them darker. If you want to mutate `self` instead of copying, see [`Self::darken()`]
    pub fn darkened(&self, amount: f32) -> PreonColor {
        let linear = self.into_linear();

        PreonColor::from_rgba(
            linear.r * (1.0 - amount),
            linear.g * (1.0 - amount),
            linear.b * (1.0 - amount),
            self.a,
        )
    }

    /// Get the value of the PreonColor without color space conversion applied.
    ///
    /// ### Performance
    ///
    /// This function reverts the conversion every time it is called, due to it being a pretty expensive operation, using this at runtime is discouraged
    pub fn into_linear(&self) -> PreonColor {
        PreonColor {
            r: libm::powf(self.r, 1.0 / 2.2),
            g: libm::powf(self.g, 1.0 / 2.2),
            b: libm::powf(self.b, 1.0 / 2.2),
            a: self.a,
        }
    }

    /// Break a copy of `self` into a tuple of 4 f32's (r, g, b, a)
    pub fn into_f32_tuple(&self) -> (f32, f32, f32, f32) {
        (self.r, self.g, self.b, self.a)
    }

    /// Break a copy of `self` into a tuple of 4 f64's (r, g, b, a)
    pub fn into_f64_tuple(&self) -> (f64, f64, f64, f64) {
        (self.r as f64, self.g as f64, self.b as f64, self.a as f64)
    }

    pub fn into_rgba8_tuple(&self) -> (u8, u8, u8, u8) {
        (
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8,
            (self.a * 255.0) as u8,
        )
    }

    pub fn into_hex(&self) -> String {
        let (r, g, b, a) = self.into_linear().into_rgba8_tuple();
        format!("#{:02x}{:02x}{:02x}{:02x}", r, g, b, a)
    }
}

impl Display for PreonColor {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.into_hex())
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

    #[inline(always)]
    pub fn from_xy(x: i32, y: i32) -> PreonBorder {
        PreonBorder::new(y, x, y, x)
    }

    #[inline(always)]
    pub fn from_single(value: i32) -> PreonBorder {
        PreonBorder::new(value, value, value, value)
    }

    #[inline(always)]
    pub fn zero() -> PreonBorder {
        PreonBorder::from_single(0)
    }

    #[inline(always)]
    pub fn x(&self) -> i32 {
        self.left + self.right
    }

    #[inline(always)]
    pub fn y(&self) -> i32 {
        self.top + self.bottom
    }

    #[inline(always)]
    pub fn top_left(&self) -> PreonVector<i32> {
        PreonVector::new(self.left, self.top)
    }

    #[inline(always)]
    pub fn top_right(&self) -> PreonVector<i32> {
        PreonVector::new(self.right, self.top)
    }

    #[inline(always)]
    pub fn bottom_left(&self) -> PreonVector<i32> {
        PreonVector::new(self.left, self.bottom)
    }

    #[inline(always)]
    pub fn bottom_right(&self) -> PreonVector<i32> {
        PreonVector::new(self.right, self.bottom)
    }
}

impl Display for PreonBorder {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "t: {}, r: {}, b: {}, l: {}",
            self.top, self.right, self.bottom, self.left
        )
    }
}

impl Add<PreonBorder> for PreonVector<i32> {
    type Output = PreonVector<i32>;

    fn add(self, rhs: PreonBorder) -> Self::Output {
        PreonVector::new(self.x + rhs.x(), self.y + rhs.y())
    }
}

impl Sub<PreonBorder> for PreonVector<i32> {
    type Output = PreonVector<i32>;

    fn sub(self, rhs: PreonBorder) -> Self::Output {
        PreonVector::new(self.x - rhs.x(), self.y - rhs.y())
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct PreonCorners {
    pub top_left: f32,
    pub top_right: f32,
    pub bottom_right: f32,
    pub bottom_left: f32,
}

impl PreonCorners {
    pub const ZERO: PreonCorners = PreonCorners {
        top_left: 0.0,
        top_right: 0.0,
        bottom_right: 0.0,
        bottom_left: 0.0,
    };

    pub fn new(top_left: f32, top_right: f32, bottom_right: f32, bottom_left: f32) -> PreonCorners {
        PreonCorners {
            top_left,
            top_right,
            bottom_right,
            bottom_left,
        }
    }

    pub fn from_xy(x: f32, y: f32) -> PreonCorners {
        PreonCorners::new(y, x, y, x)
    }

    pub fn from_single(value: f32) -> PreonCorners {
        PreonCorners::new(value, value, value, value)
    }

    pub fn pill(rect: PreonVector<f32>) -> PreonCorners {
        if rect.x > rect.y {
            PreonCorners::from_single(rect.y * 0.5)
        } else {
            PreonCorners::from_single(rect.x * 0.5)
        }
    }
}

impl Display for PreonCorners {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "tl: {}, tr: {}, bl: {}, br: {}",
            self.top_left, self.top_right, self.bottom_left, self.bottom_right
        )
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PreonAlignment {
    Start,
    Center,
    End,
    Spread,
}

impl From<u8> for PreonAlignment {
    fn from(num: u8) -> Self {
        match num {
            0 => Self::Start,
            1 => Self::Center,
            2 => Self::End,
            3 => Self::Spread,
            _ => panic!("Unrecognised alignment value!"),
        }
    }
}

impl Default for PreonAlignment {
    fn default() -> Self {
        Self::Start
    }
}

impl Display for PreonAlignment {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PreonAlignment::Start => "Start",
                PreonAlignment::Center => "Center",
                PreonAlignment::End => "End",
                PreonAlignment::Spread => "Spread",
            }
        )
    }
}
