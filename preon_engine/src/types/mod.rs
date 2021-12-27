use core::fmt::Display;

#[derive(Default, Debug, Clone, Copy)]
pub struct Color(pub f32, pub f32, pub f32, pub f32);

impl Display for Color {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "#{:02X}{:02X}{:02X}{:02X}",
            (self.0 * 255.0) as u8,
            (self.1 * 255.0) as u8,
            (self.2 * 255.0) as u8,
            (self.3 * 255.0) as u8
        )
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Position(i32, i32);

#[derive(Default, Debug, Clone, Copy)]
pub struct Size(i32, i32);

/// (top left, top right, bottom left, bottom right)
#[derive(Default, Debug, Clone, Copy)]
pub struct Corners(f32, f32, f32, f32);
