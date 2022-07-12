use crate::{transform::vector::Vec4, powf};

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Color {
    srgb_r: f32,
    srgb_g: f32,
    srgb_b: f32,
    srgb_a: f32,
}

impl Color {
    #[inline]
    pub fn from_srgb_rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color {
            srgb_r: r,
            srgb_g: g,
            srgb_b: b,
            srgb_a: a,
        }
    }

    #[inline]
    pub fn from_srgb_rgb(r: f32, g: f32, b: f32) -> Color {
        Color {
            srgb_r: r,
            srgb_g: g,
            srgb_b: b,
            srgb_a: 1.0,
        }
    }

    #[inline]
    pub fn from_rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color {
            srgb_r: powf!(r, 2.2),
            srgb_g: powf!(g, 2.2),
            srgb_b: powf!(b, 2.2),
            srgb_a: powf!(a, 2.2),
        }
    }

    #[inline]
    pub fn from_rgb(r: f32, g: f32, b: f32) -> Color {
        Color {
            srgb_r: powf!(r, 2.2),
            srgb_g: powf!(g, 2.2),
            srgb_b: powf!(b, 2.2),
            srgb_a: 1.0,
        }
    }

    #[inline]
    pub fn from_rgba8(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color::from_rgba(
            r as f32 / 255.0,
            g as f32 / 255.0,
            b as f32 / 255.0,
            a as f32 / 255.0,
        )
    }

    #[inline]
    pub fn from_rgb8(r: u8, g: u8, b: u8) -> Color {
        Color::from_rgb(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
    }

    #[inline]
    pub fn r(self) -> f32 {
        self.srgb_r
    }

    #[inline]
    pub fn g(self) -> f32 {
        self.srgb_g
    }

    #[inline]
    pub fn b(self) -> f32 {
        self.srgb_b
    }

    #[inline]
    pub fn a(self) -> f32 {
        self.srgb_a
    }

    #[inline]
    pub fn rgba(self) -> (f32, f32, f32, f32) {
        (self.r(), self.g(), self.b(), self.a())
    }
    
    #[inline]
    pub fn rgb(self) -> (f32, f32, f32) {
        (self.r(), self.g(), self.b())
    }
}

impl From<Color> for Vec4 {
    fn from(color: Color) -> Vec4 {
        Vec4(color.srgb_r, color.srgb_g, color.srgb_b, color.srgb_a)
    }
}
