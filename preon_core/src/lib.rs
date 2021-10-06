use std::{convert::TryInto, ops::{Add, Div, Mul, Sub}};

pub struct PreonCore {
    pub root: PreonVertical,
}

impl PreonCore {
    pub fn init() -> Self {
        Self {
            root: PreonVertical::new(),
        }
    }

    pub fn update(&mut self) {}
}

pub trait PreonRenderer {
    fn start(&mut self, core: &PreonCore);
    fn update(&mut self, core: &mut PreonCore) -> bool;
    fn render(&mut self, core: &PreonCore);
}

pub trait PreonComponent {
    fn add_child(&mut self, new_child: Box<dyn PreonComponent>);
    fn set_position(&mut self, position: Vector2)
    fn layout(&mut self) -> PreonLayout;
}

// Used by PreonRenderers to make their own trait
pub trait PreonRenderableComponent<T: PreonRenderer> {}

pub struct PreonRect {
    pub layout: PreonLayout,
    pub color: (f32, f32, f32, f32),
}

impl PreonRect {
    pub fn new() -> Self {
        Self {
            layout: PreonLayout {
                margin: m(0),
                padding: p(0),
                min_width: 0,
                min_height: 0,
                size_flags: SF_FILL_EXPAND,
            },
            color: color(0xda0037ff),
        }
    }
}

impl PreonComponent for PreonRect {
    fn add_child(&mut self, _new_child: Box<dyn PreonComponent>) {
        panic!("PreonRect is not made to hold children!")
    }

    fn layout(&mut self) -> PreonLayout { self.layout }
}

pub struct PreonVertical {
    pub layout: PreonLayout,
    pub children: Vec<Box<dyn PreonComponent>>,
}

impl PreonVertical {
    pub fn new() -> Self {
        Self {
            layout: PreonLayout {
                margin: m(0),
                padding: p(0),
                min_width: 0,
                min_height: 0,
                size_flags: SF_FILL,
            },
            children: Vec::new()
        }
    }
}

impl PreonComponent for PreonVertical {
    fn add_child(&mut self, new_child: Box<dyn PreonComponent>) {
        self.children.push(new_child);
    }

    fn layout(&mut self) -> PreonLayout {
        for child in self.children.iter_mut() {
            child.layout();
        }

        self.layout
    }
}

pub const SU8: usize = 1usize;
pub const SU16: usize = 2usize;
pub const SU32: usize = 4usize;
pub const SU64: usize = 8usize;
pub const SU128: usize = 16usize;
pub const SI8: usize = 1usize;
pub const SI16: usize = 2usize;
pub const SI32: usize = 4usize;
pub const SI64: usize = 8usize;
pub const SI128: usize = 16usize;
pub const SF32: usize = 4usize;
pub const SF64: usize = 8usize;
pub const SBOOL: usize = 1usize;
pub const SBOOLS: usize = 1usize;

pub const SF_HORIZONTAL_FILL: u8 = 0b00000001;
pub const SF_HORIZONTAL_EXPAND: u8 = 0b00000010;
pub const SF_VERTICAL_FILL: u8 = 0b00000100;
pub const SF_VERTICAL_EXPAND: u8 = 0b00001000;

pub const SF_VERTICAL_FILL_EXPAND: u8 = SF_VERTICAL_FILL + SF_VERTICAL_EXPAND;
pub const SF_HORIZONTAL_FILL_EXPAND: u8 = SF_HORIZONTAL_FILL + SF_HORIZONTAL_EXPAND;

pub const SF_EXPAND: u8 = SF_HORIZONTAL_EXPAND + SF_VERTICAL_EXPAND;
pub const SF_FILL: u8 = SF_HORIZONTAL_FILL + SF_VERTICAL_FILL;
pub const SF_FILL_EXPAND: u8 = SF_FILL + SF_EXPAND;

#[inline(always)]
#[cfg(target_endian = "little")]
pub fn color(c: u32) -> (f32, f32, f32, f32) {
    (
        f32::from((c >> 24) as u8) / 255f32,
        f32::from((c >> 16) as u8) / 255f32,
        f32::from((c >> 8) as u8) / 255f32,
        f32::from(c as u8) / 255f32,
    )
}

#[inline(always)]
#[cfg(target_endian = "big")]
pub fn color(c: u32) -> (f32, f32, f32, f32) {
    (
        f32::from(c as u8) / 255f32,
        f32::from((c >> 8) as u8) / 255f32,
        f32::from((c >> 16) as u8) / 255f32,
        f32::from((c >> 24) as u8) / 255f32,
    )
}

pub struct PreonData {
    raw: Vec<u8>,
}

impl PreonData {
    pub fn new(size: usize) -> Self {
        let mut vect: Vec<u8> = Vec::new();
        for _ in 0..size {
            vect.push(0u8);
        }

        Self { raw: vect }
    }

    // Sorry, couldn't get the compiler to cooperate

    #[inline(always)]
    pub fn set_u8(&mut self, offset: usize, value: u8) {
        self.raw[offset] = value
    }
    #[inline(always)]
    pub fn set_u16(&mut self, offset: usize, value: u16) {
        let b = value.to_ne_bytes();
        for i in 0..SU16 {
            self.raw[i + offset] = b[i]
        }
    }
    #[inline(always)]
    pub fn set_u32(&mut self, offset: usize, value: u32) {
        let b = value.to_ne_bytes();
        for i in 0..SU32 {
            self.raw[i + offset] = b[i]
        }
    }
    #[inline(always)]
    pub fn set_u64(&mut self, offset: usize, value: u64) {
        let b = value.to_ne_bytes();
        for i in 0..SU64 {
            self.raw[i + offset] = b[i]
        }
    }
    #[inline(always)]
    pub fn set_u128(&mut self, offset: usize, value: u128) {
        let b = value.to_ne_bytes();
        for i in 0..SU128 {
            self.raw[i + offset] = b[i]
        }
    }
    #[inline(always)]
    pub fn set_i8(&mut self, offset: usize, value: i8) {
        self.raw[offset] = value as u8;
    }
    #[inline(always)]
    pub fn set_i16(&mut self, offset: usize, value: i16) {
        let b = value.to_ne_bytes();
        for i in 0..SI16 {
            self.raw[i + offset] = b[i]
        }
    }
    #[inline(always)]
    pub fn set_i32(&mut self, offset: usize, value: i32) {
        let b = value.to_ne_bytes();
        for i in 0..SI32 {
            self.raw[i + offset] = b[i]
        }
    }
    #[inline(always)]
    pub fn set_i64(&mut self, offset: usize, value: i64) {
        let b = value.to_ne_bytes();
        for i in 0..SI64 {
            self.raw[i + offset] = b[i]
        }
    }
    #[inline(always)]
    pub fn set_i128(&mut self, offset: usize, value: i128) {
        let b = value.to_ne_bytes();
        for i in 0..SI128 {
            self.raw[i + offset] = b[i]
        }
    }
    #[inline(always)]
    pub fn set_f32(&mut self, offset: usize, value: f32) {
        let b = value.to_ne_bytes();
        for i in 0..SF32 {
            self.raw[i + offset] = b[i]
        }
    }
    #[inline(always)]
    pub fn set_f64(&mut self, offset: usize, value: f64) {
        let b = value.to_ne_bytes();
        for i in 0..SF64 {
            self.raw[i + offset] = b[i]
        }
    }
    #[inline(always)]
    pub fn set_bool(&mut self, offset: usize, value: bool) {
        self.raw[offset] = value as u8;
    }
    #[inline(always)]
    #[cfg(target_endian = "little")]
    pub fn set_bools(&mut self, offset: usize, value: [bool; 8]) {
        self.raw[offset] = 0;
        for index in 0..8 {
            // Explanation (example of iteration 5 of the for-loop):
            //    1. get bool (true) as u8 (1, 0b00000001 in binary)
            //    2. shift to left <index> times (0b00010000),
            //    3. byte-or with current value (0b00010000 | ob00001001 = 0b00011001)

            self.raw[offset] = self.raw[offset] | (value[index] as u8) << index;
        }
    }
    #[inline(always)]
    #[cfg(target_endian = "big")]
    pub fn set_bools(&mut self, offset: usize, value: [bool; 8]) {
        self.raw[offset] = 0;
        for index in 0..8 {
            // See set_bools little endian for explanation,
            // it's the same thing but other way for big endian

            self.raw[offset] = self.raw[offset] | (value[index] as u8) >> index;
        }
    }

    #[inline(always)]
    pub fn get_u8(&self, offset: usize) -> u8 {
        self.raw.get(offset).unwrap().to_owned()
    }
    #[inline(always)]
    pub fn get_u16(&self, offset: usize) -> u16 {
        u16::from_ne_bytes(
            self.raw
                .get(offset..offset + SU16)
                .unwrap()
                .to_owned()
                .try_into()
                .unwrap(),
        )
    }
    #[inline(always)]
    pub fn get_u32(&self, offset: usize) -> u32 {
        u32::from_ne_bytes(
            self.raw
                .get(offset..offset + SU32)
                .unwrap()
                .to_owned()
                .try_into()
                .unwrap(),
        )
    }
    #[inline(always)]
    pub fn get_u64(&self, offset: usize) -> u64 {
        u64::from_ne_bytes(
            self.raw
                .get(offset..offset + SU64)
                .unwrap()
                .to_owned()
                .try_into()
                .unwrap(),
        )
    }
    #[inline(always)]
    pub fn get_u128(&self, offset: usize) -> u128 {
        u128::from_ne_bytes(
            self.raw
                .get(offset..offset + SU128)
                .unwrap()
                .to_owned()
                .try_into()
                .unwrap(),
        )
    }
    #[inline(always)]
    pub fn get_i8(&self, offset: usize) -> i8 {
        self.raw.get(offset).unwrap().to_owned() as i8
    }
    #[inline(always)]
    pub fn get_i16(&self, offset: usize) -> i16 {
        i16::from_ne_bytes(
            self.raw
                .get(offset..offset + SI16)
                .unwrap()
                .to_owned()
                .try_into()
                .unwrap(),
        )
    }
    #[inline(always)]
    pub fn get_i32(&self, offset: usize) -> i32 {
        i32::from_ne_bytes(
            self.raw
                .get(offset..offset + SI32)
                .unwrap()
                .to_owned()
                .try_into()
                .unwrap(),
        )
    }
    #[inline(always)]
    pub fn get_i64(&self, offset: usize) -> i64 {
        i64::from_ne_bytes(
            self.raw
                .get(offset..offset + SI64)
                .unwrap()
                .to_owned()
                .try_into()
                .unwrap(),
        )
    }
    #[inline(always)]
    pub fn get_i128(&self, offset: usize) -> i128 {
        i128::from_ne_bytes(
            self.raw
                .get(offset..offset + SI128)
                .unwrap()
                .to_owned()
                .try_into()
                .unwrap(),
        )
    }
    #[inline(always)]
    pub fn get_f32(&self, offset: usize) -> f32 {
        f32::from_ne_bytes(
            self.raw
                .get(offset..offset + SF32)
                .unwrap()
                .to_owned()
                .try_into()
                .unwrap(),
        )
    }
    #[inline(always)]
    pub fn get_f64(&self, offset: usize) -> f64 {
        f64::from_ne_bytes(
            self.raw
                .get(offset..offset + SF64)
                .unwrap()
                .to_owned()
                .try_into()
                .unwrap(),
        )
    }
    #[inline(always)]
    pub fn get_bool(&mut self, offset: usize) -> bool {
        self.raw[offset] == 1
    }
    #[inline(always)]
    #[cfg(target_endian = "big")]
    pub fn get_bools(&mut self, offset: usize) -> [bool; 8] {
        let mut result: [bool; 8] = [false; 8];
        for i in 0..8 {
            result[i] = (self.raw[offset] & (0b10000000 >> i)) << i == 1
        }
        result
    }
    #[inline(always)]
    #[cfg(target_endian = "little")]
    pub fn get_bools(&mut self, offset: usize) -> [bool; 8] {
        let mut result: [bool; 8] = [false; 8];
        for i in 0..8 {
            result[i] = (self.raw[offset] & (0b00000001 << i)) >> i == 1
        }
        result
    }

    pub fn free(&mut self) {
        self.raw.clear();
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PreonMargin {
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
    pub left: u32,
}

#[inline(always)]
pub fn m4(t: u32, r: u32, b: u32, l: u32) -> PreonMargin {
    PreonMargin {
        top: t,
        right: r,
        bottom: b,
        left: l,
    }
}

#[inline(always)]
pub fn m2(x: u32, y: u32) -> PreonMargin {
    PreonMargin {
        top: y,
        right: x,
        bottom: y,
        left: x,
    }
}

#[inline(always)]
pub fn mx(x: u32) -> PreonMargin {
    PreonMargin {
        top: 0,
        right: x,
        bottom: 0,
        left: x,
    }
}

#[inline(always)]
pub fn my(y: u32) -> PreonMargin {
    PreonMargin {
        top: y,
        right: 0,
        bottom: y,
        left: 0,
    }
}

#[inline(always)]
pub fn m(v: u32) -> PreonMargin {
    PreonMargin {
        top: v,
        right: v,
        bottom: v,
        left: v,
    }
}

#[inline(always)]
pub fn margin(v: u32) -> PreonMargin {
    m(v)
}
#[inline(always)]
pub fn margin_x(x: u32) -> PreonMargin {
    mx(x)
}
#[inline(always)]
pub fn margin_y(y: u32) -> PreonMargin {
    my(y)
}
#[inline(always)]
pub fn margin_xy(x: u32, y: u32) -> PreonMargin {
    m2(x, y)
}
#[inline(always)]
pub fn margin_trbl(t: u32, r: u32, b: u32, l: u32) -> PreonMargin {
    m4(t, r, b, l)
}

#[derive(Clone, Copy, Debug)]
pub struct PreonPadding {
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
    pub left: u32,
}

#[inline(always)]
pub fn p4(t: u32, r: u32, b: u32, l: u32) -> PreonPadding {
    PreonPadding {
        top: t,
        right: r,
        bottom: b,
        left: l,
    }
}

#[inline(always)]
pub fn p2(x: u32, y: u32) -> PreonPadding {
    PreonPadding {
        top: y,
        right: x,
        bottom: y,
        left: x,
    }
}

#[inline(always)]
pub fn px(x: u32) -> PreonPadding {
    PreonPadding {
        top: 0,
        right: x,
        bottom: 0,
        left: x,
    }
}

#[inline(always)]
pub fn py(y: u32) -> PreonPadding {
    PreonPadding {
        top: y,
        right: 0,
        bottom: y,
        left: 0,
    }
}

#[inline(always)]
pub fn p(v: u32) -> PreonPadding {
    PreonPadding {
        top: v,
        right: v,
        bottom: v,
        left: v,
    }
}

#[inline(always)]
pub fn padding(v: u32) -> PreonPadding {
    p(v)
}
#[inline(always)]
pub fn padding_x(x: u32) -> PreonPadding {
    px(x)
}
#[inline(always)]
pub fn padding_y(y: u32) -> PreonPadding {
    py(y)
}
#[inline(always)]
pub fn padding_xy(x: u32, y: u32) -> PreonPadding {
    p2(x, y)
}
#[inline(always)]
pub fn padding_trbl(t: u32, r: u32, b: u32, l: u32) -> PreonPadding {
    p4(t, r, b, l)
}

#[derive(Clone, Copy, Debug)]
pub struct PreonLayout {
    pub margin: PreonMargin,
    pub padding: PreonPadding,
    pub min_width: u32,
    pub min_height: u32,
    pub size_flags: u8,
}

pub struct Vector2<T: > {
    pub x: T,
    pub y: T
}

impl<T> Vector2<T> {

}