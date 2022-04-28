#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec2(pub f32, pub f32);

impl VectorMath for Vec2 {
    #[inline]
    fn length(self) -> f32 {
        (self.0.powi(2) + self.1.powi(2)).sqrt()
    }

    #[inline]
    fn normalize(&mut self) {
        let length = self.length();
        self.0 /= length;
        self.1 /= length;
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl VectorMath for Vec3 {
    #[inline]
    fn length(self) -> f32 {
        (self.0.powi(2) + self.1.powi(2) + self.2.powi(2)).sqrt()
    }

    #[inline]
    fn normalize(&mut self) {
        let length = self.length();
        self.0 /= length;
        self.1 /= length;
        self.2 /= length;
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec4(pub f32, pub f32, pub f32, pub f32);

impl VectorMath for Vec4 {
    #[inline]
    fn length(self) -> f32 {
        (self.0.powi(2) + self.1.powi(2) + self.2.powi(2) + self.3.powi(2)).sqrt()
    }

    #[inline]
    fn normalize(&mut self) {
        let length = self.length();
        self.0 /= length;
        self.1 /= length;
        self.2 /= length;
        self.3 /= length;
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
        let mut clone = self.clone();
        clone.normalize();
        clone
    }
}
