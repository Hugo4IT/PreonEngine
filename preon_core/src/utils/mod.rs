#[cfg(target_feature="utils")]

pub fn color(r: u8, g: u8, b: u8, a: u8) -> (f32, f32, f32, f32) {
    (f32::from(r)/255f32, f32::from(g)/255f32, f32::from(b)/255f32, f32::from(a)/255f32)
}