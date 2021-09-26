use std::{convert::TryInto, mem::size_of};

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

        Self {
            raw: vect,
        }
    }

    // Sorry, couldn't get the compiler to cooperate

    #[inline(always)]
    pub fn set_u8(&mut self, offset: usize, value: u8) {
        self.raw[offset] = value
    }
    #[inline(always)]
    pub fn set_u16(&mut self, offset: usize, value: u16) {
        let b = value.to_be_bytes();
        for i in 0..size_of::<u16>() {
            self.raw[i + offset] = b[i]
        }
    }
    #[inline(always)]
    pub fn set_u32(&mut self, offset: usize, value: u32) {
        let b = value.to_be_bytes();
        for i in 0..size_of::<u32>() {
            self.raw[i + offset] = b[i]
        }
    }
    #[inline(always)]
    pub fn set_u64(&mut self, offset: usize, value: u64) {
        let b = value.to_be_bytes();
        for i in 0..size_of::<u64>() {
            self.raw[i + offset] = b[i]
        }
    }
    #[inline(always)]
    pub fn set_u128(&mut self, offset: usize, value: u128) {
        let b = value.to_be_bytes();
        for i in 0..size_of::<u128>() {
            self.raw[i + offset] = b[i]
        }
    }
    #[inline(always)]
    pub fn set_i8(&mut self, offset: usize, value: i8) {
        let b = value.to_be_bytes();
        for i in 0..size_of::<i8>() {
            self.raw[i + offset] = b[i]
        }
    }
    #[inline(always)]
    pub fn set_i16(&mut self, offset: usize, value: i16) {
        let b = value.to_be_bytes();
        for i in 0..size_of::<i16>() {
            self.raw[i + offset] = b[i]
        }
    }
    #[inline(always)]
    pub fn set_i32(&mut self, offset: usize, value: i32) {
        let b = value.to_be_bytes();
        for i in 0..size_of::<i32>() {
            self.raw[i + offset] = b[i]
        }
    }
    #[inline(always)]
    pub fn set_i64(&mut self, offset: usize, value: i64) {
        let b = value.to_be_bytes();
        for i in 0..size_of::<i64>() {
            self.raw[i + offset] = b[i]
        }
    }
    #[inline(always)]
    pub fn set_i128(&mut self, offset: usize, value: i128) {
        let b = value.to_be_bytes();
        for i in 0..size_of::<i128>() {
            self.raw[i + offset] = b[i]
        }
    }
    #[inline(always)]
    pub fn set_f32(&mut self, offset: usize, value: f32) {
        let b = value.to_be_bytes();
        for i in 0..size_of::<f32>() {
            self.raw[i + offset] = b[i]
        }
    }
    #[inline(always)]
    pub fn set_f64(&mut self, offset: usize, value: f64) {
        let b = value.to_be_bytes();
        for i in 0..size_of::<f64>() {
            self.raw[i + offset] = b[i]
        }
    }
    #[inline(always)]
    pub fn set_bool(&mut self, offset: usize, value: bool) {
        let b: u8 = value.into();
        self.raw[offset] = b
    }
    #[inline(always)]
    pub fn set_bools(&mut self, offset: usize, value: [bool; 8]) {
        self.raw[offset] = 0;
        for i in 0..8 {
            self.raw[offset] = ((value[i] as u8) >> i) | self.raw[offset];
        }
    }

    #[inline(always)]
    pub fn get_u8(&self, offset: usize) -> u8 {
        self.raw.get(offset).unwrap().to_owned()
    }
    #[inline(always)]
    pub fn get_u16(&self, offset: usize) -> u16 {
        u16::from_be_bytes(
            self.raw
                .get(offset..offset + size_of::<u16>())
                .unwrap()
                .to_owned()
                .try_into()
                .unwrap(),
        )
    }
    #[inline(always)]
    pub fn get_u32(&self, offset: usize) -> u32 {
        u32::from_be_bytes(
            self.raw
                .get(offset..offset + size_of::<u32>())
                .unwrap()
                .to_owned()
                .try_into()
                .unwrap(),
        )
    }
    #[inline(always)]
    pub fn get_u64(&self, offset: usize) -> u64 {
        u64::from_be_bytes(
            self.raw
                .get(offset..offset + size_of::<u64>())
                .unwrap()
                .to_owned()
                .try_into()
                .unwrap(),
        )
    }
    #[inline(always)]
    pub fn get_u128(&self, offset: usize) -> u128 {
        u128::from_be_bytes(
            self.raw
                .get(offset..offset + size_of::<u128>())
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
        i16::from_be_bytes(
            self.raw
                .get(offset..offset + size_of::<i16>())
                .unwrap()
                .to_owned()
                .try_into()
                .unwrap(),
        )
    }
    #[inline(always)]
    pub fn get_i32(&self, offset: usize) -> i32 {
        i32::from_be_bytes(
            self.raw
                .get(offset..offset + size_of::<i32>())
                .unwrap()
                .to_owned()
                .try_into()
                .unwrap(),
        )
    }
    #[inline(always)]
    pub fn get_i64(&self, offset: usize) -> i64 {
        i64::from_be_bytes(
            self.raw
                .get(offset..offset + size_of::<i64>())
                .unwrap()
                .to_owned()
                .try_into()
                .unwrap(),
        )
    }
    #[inline(always)]
    pub fn get_i128(&self, offset: usize) -> i128 {
        i128::from_be_bytes(
            self.raw
                .get(offset..offset + size_of::<i128>())
                .unwrap()
                .to_owned()
                .try_into()
                .unwrap(),
        )
    }
    #[inline(always)]
    pub fn get_f32(&self, offset: usize) -> f32 {
        f32::from_be_bytes(
            self.raw
                .get(offset..offset + size_of::<i32>())
                .unwrap()
                .to_owned()
                .try_into()
                .unwrap(),
        )
    }
    #[inline(always)]
    pub fn get_f64(&self, offset: usize) -> f64 {
        f64::from_be_bytes(
            self.raw
                .get(offset..offset + size_of::<i64>())
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
