use crate::{sqrt, inv_sqrt};

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec2(pub f32, pub f32);

impl VectorMath for Vec2 {
    #[inline]
    fn length(self) -> f32 {
        sqrt!(self.0 * self.0 + self.1 * self.1)
    }

    #[inline]
    fn normalize(&mut self) {
        let length_mul = inv_sqrt!(self.0 * self.0 + self.1 * self.1);
        self.0 *= length_mul;
        self.1 *= length_mul;
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl VectorMath for Vec3 {
    #[inline]
    fn length(self) -> f32 {
        sqrt!(self.0 * self.0 + self.1 * self.1 + self.2 * self.2)
    }

    #[inline]
    fn normalize(&mut self) {
        let length_mul = inv_sqrt!(self.0 * self.0 + self.1 * self.1 + self.2 * self.2);
        self.0 *= length_mul;
        self.1 *= length_mul;
        self.2 *= length_mul;
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec4(pub f32, pub f32, pub f32, pub f32);

impl VectorMath for Vec4 {
    #[inline]
    fn length(self) -> f32 {
        sqrt!(self.0 * self.0 + self.1 * self.1 + self.2 * self.2 + self.3 * self.3)
    }

    #[inline]
    fn normalize(&mut self) {
        let length_mul = inv_sqrt!(self.0 * self.0 + self.1 * self.1 + self.2 * self.2 + self.3 * self.3);
        self.0 *= length_mul;
        self.1 *= length_mul;
        self.2 *= length_mul;
        self.3 *= length_mul;
    }
}

pub trait VectorMath {
    fn length(self) -> f32;
    fn normalize(&mut self);

    #[inline]
    fn normalized(self) -> Self
    where
        Self: Sized + Clone,
    {
        let mut clone = self;
        clone.normalize();
        clone
    }
}