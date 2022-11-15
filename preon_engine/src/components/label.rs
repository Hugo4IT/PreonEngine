use core::str::FromStr;

use alloc::string::String;

use crate::{components::{PreonComponentStack, PreonComponentStorage}, types::{PreonAlignment, PreonColor}, log};

use super::{PreonCustomComponentStack, PreonComponentBuilder};

/// Human-readable text configuration. After encoding it will look like this (u64):
///
/// ```txt
/// 0000000000 0 0 00 00 00000000 00000000 00000000 00000000 0000000000000000
/// ¯¯¯¯¯¯¯¯¯¯ ¯ ¯ ¯¯ ¯¯ ¯¯¯¯¯¯¯¯ ¯¯¯¯¯¯¯¯ ¯¯¯¯¯¯¯¯ ¯¯¯¯¯¯¯¯ ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯
/// |          | | |  |  |        |        |        |        |
/// |          | | |  |  |        |        |        |        `> size: u16
/// |          | | |  |  |        |        |        `> Alpha: u8
/// |          | | |  |  |        |        `> Blue: u8
/// |          | | |  |  |        `> Green: u8
/// |          | | |  |  `> Red: u8
/// |          | | |  `> vertical_align: PreonAlignment (2 bits, 4 options)
/// |          | | `> horizontal_align: PreonAlignment (2 bits, 4 options)
/// |          | `> italic: bool
/// |          `> bold: bool
/// `> font_index: u10 (rust type: u16, but can only hold 10 bits in practice)
/// ```
#[derive(Debug, Clone, Copy)]
pub struct LabelConfig {
    pub size: u16,
    pub font_index: u16,
    pub color: PreonColor,
    pub bold: bool,
    pub italic: bool,
    pub vertical_align: PreonAlignment,
    pub horizontal_align: PreonAlignment,
}

impl LabelConfig {
    pub fn encode(&self) -> u64 {
        let mut result: [u8; 8] = [0u8; 8];

        let font_index: [u8; 2] = self.font_index.to_le_bytes();
        result[0] = font_index[0];

        let flags: u8 = 0u8
            | ((font_index[1] & 0b00000011) << 7)
            | ((self.bold as u8) << 5)
            | ((self.italic as u8) << 4)
            | ((self.vertical_align as u8) << 2)
            | (self.horizontal_align as u8);
        result[1] = flags;

        let (r, g, b, a) = self.color.into_rgba8_tuple();
        result[2] = r;
        result[3] = g;
        result[4] = b;
        result[5] = a;

        let size: [u8; 2] = self.size.to_le_bytes();
        result[6] = size[0];
        result[7] = size[1];

        u64::from_le_bytes(result)
    }

    pub fn decode(input: u64) -> LabelConfig {
        let buffer: [u8; 8] = input.to_le_bytes();
        let font_index = u16::from_le_bytes([buffer[0], (buffer[1] & 0b11000000) >> 6]);
        let bold = (buffer[1] & 0b00100000) == 0b00100000;
        let italic = (buffer[1] & 0b00010000) == 0b00010000;
        let vertical_align = PreonAlignment::from((buffer[1] & 0b00001100) >> 2);
        let horizontal_align = PreonAlignment::from(buffer[1] & 0b00000011);
        let color = PreonColor::from_rgba8(buffer[2], buffer[3], buffer[4], buffer[5]);
        let size = u16::from_le_bytes([buffer[6], buffer[7]]);

        LabelConfig {
            size,
            font_index,
            color,
            bold,
            italic,
            vertical_align,
            horizontal_align,
        }
    }
}

impl Default for LabelConfig {
    fn default() -> Self {
        Self {
            size: 16,
            font_index: 0,
            color: PreonColor::from_rgba8(255, 255, 255, 255),
            bold: false,
            italic: false,
            vertical_align: PreonAlignment::Start,
            horizontal_align: PreonAlignment::Start,
        }
    }
}

pub trait AddLabel<T: PreonCustomComponentStack> {
    fn start_label(&mut self, text: String) -> &mut PreonComponentBuilder<T>;
    fn start_label_str(&mut self, text: &'static str) -> &mut PreonComponentBuilder<T>;
    fn empty_label(&mut self, text: String) -> &mut PreonComponentBuilder<T>;
    fn empty_label_str(&mut self, text: &'static str) -> &mut PreonComponentBuilder<T>;
    fn start_label_cfg(&mut self, text: String, config: LabelConfig) -> &mut PreonComponentBuilder<T>;
    fn bold(&mut self) -> &mut PreonComponentBuilder<T>;
    fn italic(&mut self) -> &mut PreonComponentBuilder<T>;
}

impl<T: PreonCustomComponentStack> AddLabel<T> for PreonComponentBuilder<T> {
    fn start_label(&mut self, text: String) -> &mut PreonComponentBuilder<T> {
        self.start_label_cfg(text, LabelConfig::default())
    }

    fn start_label_str(&mut self, text: &'static str) -> &mut PreonComponentBuilder<T> {
        self.start_label(String::from_str(text).unwrap())
    }

    fn empty_label(&mut self, text: String) -> &mut PreonComponentBuilder<T> {
        self.start_label(text).end()
    }

    fn empty_label_str(&mut self, text: &'static str) -> &mut PreonComponentBuilder<T> {
        self.start_label_str(text).end()
    }

    fn start_label_cfg(&mut self, text: String, config: LabelConfig) -> &mut PreonComponentBuilder<T> {
        self.stack.push(PreonComponentStorage {
            data: PreonComponentStack::Label {
                text,
                text_settings: config.encode(),
            },
            ..Default::default()
        });

        self
    }

    fn bold(&mut self) -> &mut PreonComponentBuilder<T> {
        if let PreonComponentStack::Label {
            ref mut text_settings,
            ..
        } = self.current_mut().data
        {
            *text_settings |= 0b0000000000100000000000000000000000000000000000000000000000000000;
        }

        self
    }

    fn italic(&mut self) -> &mut PreonComponentBuilder<T> {
        if let PreonComponentStack::Label {
            ref mut text_settings,
            ..
        } = self.current_mut().data
        {
            *text_settings |= 0b0000000000010000000000000000000000000000000000000000000000000000;
        }

        self
    }
}