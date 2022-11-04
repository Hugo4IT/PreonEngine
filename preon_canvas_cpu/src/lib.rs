#![no_std]
#![allow(clippy::missing_safety_doc)]

extern crate alloc;

use alloc::{vec::Vec, vec};
use fontdue::{Font, layout::{Layout, CoordinateSystem, TextStyle}, FontSettings};

use preon_engine::loaders::image::Image;

const COLOR_MULT_LOOKUP_TABLE: &[u8; 131072] = include_bytes!("../res/color-mult-lookup-table.bin");
const COLOR_MULT_LOOKUP_TABLE_PTR: *const u8 = COLOR_MULT_LOOKUP_TABLE.as_ptr();
const COLOR_DIV_LOOKUP_TABLE: &[u8; 131072] = include_bytes!("../res/color-div-lookup-table.bin");
const COLOR_DIV_LOOKUP_TABLE_PTR: *const u8 = COLOR_DIV_LOOKUP_TABLE.as_ptr();

#[cfg(target_endian = "little")]
macro_rules! color_expr {
    (mul $x:expr, $y:expr) => (unsafe { (*((COLOR_MULT_LOOKUP_TABLE_PTR as usize + (($x) as usize * 256 + ($y) as usize) as usize * 2) as *const u16)) });
    (div $x:expr, $y:expr) => (unsafe { (*((COLOR_DIV_LOOKUP_TABLE_PTR as usize + (($x) as usize * 256 + ($y) as usize) as usize * 2) as *const u16)) });
}

#[cfg(target_endian = "big")]
compile_error!("Sorry, preon_canvas_cpu doesn't support big endian yet :/");

#[derive(Debug)]
pub enum GetPixelError {
    OutOfBounds,
}

pub struct Renderer {
    framebuffer: *mut u8,
    backbuffer: Vec<u8>,
    width: usize,
    height: usize,
    pub clear_color: u32,
    fonts: Vec<Font>,
}

impl Renderer {
    pub fn new(framebuffer: *mut u8, width: usize, height: usize) -> Renderer {
        Renderer {
            backbuffer: vec![0; width * height * 4],
            framebuffer,
            width,
            height,
            clear_color: 0xff000000,
            fonts: Vec::new(),
        }
    }

    pub fn add_font(&mut self, font: &[u8]) -> usize {
        self.fonts.push(Font::from_bytes(font, FontSettings::default()).unwrap());
        self.fonts.len() - 1
    }

    fn convert_glyph_texture(&self, texture: Vec<u8>, color: u32) -> Vec<u32> {
        texture.into_iter().map(|l| ((l as u32)<<24)|(color&0x00FFFFFF)).collect::<Vec<u32>>()
    }

    pub fn draw_char(&mut self, font: usize, x: usize, y: usize, ch: char, size: f32, color: u32) {
        let (metrics, texture) = self.fonts[font].rasterize(ch, size);
        self.blit_texture_blend(x, y, metrics.width, metrics.height, self.convert_glyph_texture(texture, color).as_slice())
    }

    pub unsafe fn draw_char_unchecked(&mut self, font: usize, x: usize, y: usize, ch: char, size: f32, color: u32) {
        let (metrics, texture) = self.fonts[font].rasterize(ch, size);
        self.blit_texture_blend_unchecked(x, y, metrics.width, metrics.height, self.convert_glyph_texture(texture, color).as_slice())
    }

    pub fn draw_string(&mut self, font: usize, x: usize, y: usize, string: &str, size: f32, color: u32) {
        let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
        layout.append(self.fonts.as_slice(), &TextStyle::new(string, size, font));

        for glyph in layout.glyphs() {
            let (metrics, texture) = self.fonts[glyph.font_index].rasterize_config(glyph.key);
            self.blit_texture_blend(
                x + glyph.x as usize,
                y + glyph.y as usize,
                metrics.width,
                metrics.height,
                self.convert_glyph_texture(texture, color).as_slice()
            )
        }
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }

    pub fn fill_rect(&mut self, x: usize, y: usize, width: usize, height: usize, color: u32) {
        if x + width <= self.width && y + height <= self.height {
            unsafe { self.fill_rect_unchecked(x, y, width, height, color) }
        }
    }

    pub unsafe fn fill_rect_unchecked(&mut self, x: usize, y: usize, width: usize, height: usize, color: u32) {
        let offset = x * 4;
        let start = y * self.width * 4;
        let buffer = self.backbuffer.as_mut_ptr() as usize + start;

        for y in 0..height {
            for x in 0..width {
                core::ptr::write(
                    (buffer + offset + x * 4 + y * self.width * 4) as *mut u32,
                    color,
                )
            }
        }
    }

    pub fn fill_rect_blend(&mut self, x: usize, y: usize, width: usize, height: usize, color: u32) {
        if x + width <= self.width && y + height <= self.height {
            unsafe { self.fill_rect_blend_unchecked(x, y, width, height, color) }
        }
    }

    pub unsafe fn fill_rect_blend_unchecked(&mut self, x: usize, y: usize, width: usize, height: usize, color: u32) {
        let offset = x * 4;
        let start = y * self.width * 4;
        let buffer = self.backbuffer.as_mut_ptr() as usize + start;

        for y in 0..height {
            for x in 0..width {
                let dst = (buffer + offset + x * 4 + y * self.width * 4) as *mut u32;
                let framebuffer_color = core::ptr::read(dst as *const u32);

                core::ptr::write(
                    dst,
                    Self::overlay_color(framebuffer_color, color),
                )
            }
        }
    }

    pub fn fill_round_rect(&mut self, x: usize, y: usize, width: usize, height: usize, color: u32, radius: usize) {
        if x + width <= self.width && y + height <= self.height && radius <= width / 2 && radius <= height / 2 {
            unsafe { self.fill_round_rect_unchecked(x, y, width, height, color, radius) }
        }
    }

    pub unsafe fn fill_round_rect_unchecked(&mut self, x: usize, y: usize, width: usize, height: usize, color: u32, radius: usize) {
        let offset = x * 4;
        let start = y * self.width * 4;
        let buffer = self.backbuffer.as_mut_ptr() as usize + start;

        for y in 0..height {
            if y < radius as usize {
                // Use pythagorean theorem to figure out width of circle at given y position
                let corner_height = radius - y;
                let corner = radius - preon_engine::sqrt!((radius * radius - corner_height * corner_height) as f32) as usize;

                for x in 0..(width-corner-corner) {
                    core::ptr::write(
                        (buffer + offset + (x+corner) * 4 + y * self.width * 4) as *mut u32,
                        color,
                    )
                }
            } else if y >= height - radius as usize {
                // Use pythagorean theorem to figure out width of circle at given y position
                let corner_height = y + radius - height + 1;
                let corner = radius - preon_engine::sqrt!((radius * radius - corner_height * corner_height) as f32) as usize;

                for x in 0..(width-corner-corner) {
                    core::ptr::write(
                        (buffer + offset + (x+corner) * 4 + y * self.width * 4) as *mut u32,
                        color,
                    )
                }
            } else {
                for x in 0..width {
                    core::ptr::write(
                        (buffer + offset + x * 4 + y * self.width * 4) as *mut u32,
                        color,
                    )
                }
            }
        }
    }

    pub fn fill_round_rect_blend(&mut self, x: usize, y: usize, width: usize, height: usize, color: u32, radius: usize) {
        if x + width <= self.width && y + height <= self.height && radius <= width / 2 && radius <= height / 2 {
            unsafe { self.fill_round_rect_blend_unchecked(x, y, width, height, color, radius) }
        }
    }

    pub unsafe fn fill_round_rect_blend_unchecked(&mut self, x: usize, y: usize, width: usize, height: usize, color: u32, radius: usize) {
        let offset = x * 4;
        let start = y * self.width * 4;
        let buffer = self.backbuffer.as_mut_ptr() as usize + start;

        for y in 0..height {
            if y < radius as usize {
                // Use pythagorean theorem to figure out width of circle at given y position
                let corner_height = radius - y;
                let corner = radius - preon_engine::sqrt!((radius * radius - corner_height * corner_height) as f32) as usize;

                for x in 0..(width-corner-corner) {
                    let dst = (buffer + offset + (x+corner) * 4 + y * self.width * 4) as *mut u32;
                    let framebuffer_color = core::ptr::read(dst as *const u32);

                    core::ptr::write(
                        dst,
                        Self::overlay_color(framebuffer_color, color),
                    )
                }
            } else if y >= height - radius as usize {
                // Use pythagorean theorem to figure out width of circle at given y position
                let corner_height = y + radius - height + 1;
                let corner = radius - preon_engine::sqrt!((radius * radius - corner_height * corner_height) as f32) as usize;

                for x in 0..(width-corner-corner) {
                    let dst = (buffer + offset + (x+corner) * 4 + y * self.width * 4) as *mut u32;
                    let framebuffer_color = core::ptr::read(dst as *const u32);
                    
                    core::ptr::write(
                        dst,
                        Self::overlay_color(framebuffer_color, color),
                    )
                }
            } else {
                for x in 0..width {
                    let dst = (buffer + offset + x * 4 + y * self.width * 4) as *mut u32;
                    let framebuffer_color = core::ptr::read(dst as *const u32);

                    core::ptr::write(
                        dst,
                        Self::overlay_color(framebuffer_color, color),
                    )
                }
            }
        }
    }

    #[inline]
    pub fn blit_image<I: Image>(&mut self, x: usize, y: usize, image: &I) {
        self.blit_texture(x, y, image.get_width(), image.get_height(), image.get_texture().as_slice())
    }

    #[inline]
    pub unsafe fn blit_image_unchecked<I: Image>(&mut self, x: usize, y: usize, image: &I) {
        self.blit_texture_unchecked(x, y, image.get_width(), image.get_height(), image.get_texture().as_slice())
    }

    #[inline]
    pub fn blit_image_blend<I: Image>(&mut self, x: usize, y: usize, image: &I) {
        self.blit_texture_blend(x, y, image.get_width(), image.get_height(), image.get_texture().as_slice())
    }

    #[inline]
    pub unsafe fn blit_image_blend_unchecked<I: Image>(&mut self, x: usize, y: usize, image: &I) {
        self.blit_texture_blend_unchecked(x, y, image.get_width(), image.get_height(), image.get_texture().as_slice())
    }

    pub fn blit_texture(&mut self, x: usize, y: usize, width: usize, height: usize, texture: &[u32]) {
        if x + width <= self.width && y + height <= self.height {
            unsafe { self.blit_texture_unchecked(x, y, width, height, texture) }
        }
    }

    pub unsafe fn blit_texture_unchecked(&mut self, x: usize, y: usize, width: usize, height: usize, texture: &[u32]) {
        let offset = x * 4;
        let start = y * self.width * 4;
        let buffer = self.backbuffer.as_mut_ptr() as usize + start;

        for y in 0..height {
            core::ptr::copy_nonoverlapping(
                &texture[y * width] as *const u32,
                (buffer + offset + y * self.width * 4) as *mut u32,
                width,
            )
        }
    }

    pub fn blit_texture_blend(&mut self, x: usize, y: usize, width: usize, height: usize, texture: &[u32]) {
        if x + width <= self.width && y + height <= self.height {
            unsafe { self.blit_texture_blend_unchecked(x, y, width, height, texture) }
        }
    }

    pub unsafe fn blit_texture_blend_unchecked(&mut self, x: usize, y: usize, width: usize, height: usize, texture: &[u32]) {
        let offset = x * 4;
        let start = y * self.width * 4;
        let buffer = self.backbuffer.as_mut_ptr() as usize + start;

        for y in 0..height {
            for x in 0..width {
                let index = offset + (x + y * self.width) * 4;
                let dst = buffer + index;
                
                let texture_color = texture[y * width + x];
                let framebuffer_color = core::ptr::read(dst as *const u32);
                core::ptr::write(dst as *mut u32, Self::overlay_color(framebuffer_color, texture_color));
            }
        }
    }

    #[inline]
    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x <= self.width && y <= self.height {
            unsafe { self.set_pixel_unchecked(x, y, color) }
        }
    }

    #[inline]
    pub unsafe fn set_pixel_unchecked(&mut self, x: usize, y: usize, color: u32) {
        let index = (x + y * self.width) * 4;
        core::ptr::write((self.backbuffer.as_mut_ptr() as usize + index) as *mut u32, color)
    }

    #[must_use]
    #[inline]
    pub fn get_pixel(&self, x: usize, y: usize) -> Result<(), GetPixelError> {
        if x <= self.width && y <= self.height {
            unsafe { self.get_pixel_unchecked(x, y) };
            Ok(())
        } else {
            Err(GetPixelError::OutOfBounds)
        }
    }

    #[inline]
    pub unsafe fn get_pixel_unchecked(&self, x: usize, y: usize) -> u32 {
        let index = (x + y * self.width) * 4;
        core::ptr::read((self.backbuffer.as_ptr() as usize + index) as *mut u32)
    }

    pub fn clear(&mut self) {
        let buffer = self.backbuffer.as_mut_ptr() as usize;
        for i in 0..(self.width * self.height) {
            unsafe { core::ptr::write((buffer + i * 4) as *mut u32, self.clear_color) };
        }
    }

    /// Copy backend's backbuffer over to the framebuffer
    pub fn present(&mut self) {
        unsafe {
            core::ptr::copy_nonoverlapping(self.backbuffer.as_ptr(), self.framebuffer, self.width * self.height * 4);
        };
    }

    #[allow(unused_unsafe)] // color_expr! will throw an error when not in an unsafe block, but a warning when inside one
    pub fn overlay_color(background: u32, foreground: u32) -> u32 {
        let [fg_b, fg_g, fg_r, fg_a] = foreground.to_le_bytes();
        let [bg_b, bg_g, bg_r, bg_a] = background.to_le_bytes();
            
        if fg_a == 0 {
            background
        } else if fg_a == 255 {
            foreground
        } else {
            let (fg_r, fg_g, fg_b, fg_a) = (fg_r as u16, fg_g as u16, fg_b as u16, fg_a as u16);
            let (bg_r, bg_g, bg_b, bg_a) = (bg_r as u16, bg_g as u16, bg_b as u16, bg_a as u16);

            let value_a = fg_a + color_expr!(mul bg_a, (255 - fg_a));
            let value_r = color_expr!(div color_expr!(mul fg_r, fg_a) + color_expr!(mul color_expr!(mul bg_r, bg_a), (255 - fg_a)), value_a);
            let value_g = color_expr!(div color_expr!(mul fg_g, fg_a) + color_expr!(mul color_expr!(mul bg_g, bg_a), (255 - fg_a)), value_a);
            let value_b = color_expr!(div color_expr!(mul fg_b, fg_a) + color_expr!(mul color_expr!(mul bg_b, bg_a), (255 - fg_a)), value_a);

            u32::from_le_bytes([value_b.min(255) as u8, value_g.min(255) as u8, value_r.min(255) as u8, value_a.min(255) as u8])
        }
    }

    #[allow(unused_unsafe)] // color_expr! will throw an error when not in an unsafe block, but a warning when inside one
    pub fn multiply_color(background: u32, foreground: u32) -> u32 {
        let [fg_b, fg_g, fg_r, fg_a] = foreground.to_le_bytes();
        let [bg_b, bg_g, bg_r, bg_a] = background.to_le_bytes();
            
        // let (fg_r, fg_g, fg_b, fg_a) = (fg_r as u16, fg_g as u16, fg_b as u16, fg_a as u16);
        // let (bg_r, bg_g, bg_b, bg_a) = (bg_r as u16, bg_g as u16, bg_b as u16, bg_a as u16);

        let value_a = color_expr!(mul bg_a, fg_a);
        let value_r = color_expr!(mul bg_r, fg_r);
        let value_g = color_expr!(mul bg_g, fg_g);
        let value_b = color_expr!(mul bg_b, fg_b);

        u32::from_le_bytes([value_b.min(255) as u8, value_g.min(255) as u8, value_r.min(255) as u8, value_a.min(255) as u8])
    }
}