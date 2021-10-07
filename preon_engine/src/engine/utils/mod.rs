use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

use super::types::Vector2;

#[inline(always)]
pub fn color(c: u32) -> (f32, f32, f32, f32) {
    (
        f32::from((c >> 24) as u8) / 255f32,
        f32::from((c >> 16) as u8) / 255f32,
        f32::from((c >> 8) as u8) / 255f32,
        f32::from(c as u8) / 255f32,
    )
}

/// Utility function for creating a Vector2 where both x and y are the same value
#[inline(always)]
pub fn vector2<T>(v: T) -> Vector2<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Copy
        + Clone
        + Display,
{
    Vector2::<T> { x: v, y: v }
}
