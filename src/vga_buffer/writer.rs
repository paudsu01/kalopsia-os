use crate::vga_buffer::VGA_ROWS;

use super::{ColorMode, TextColor, VGABuffer, VGAChar, VGA_COLS};

#[allow(unused)]
pub struct VGAWriter {
    // No row since we are only going to be writing at the last row
    col: u16,
    color: ColorMode,
    buffer: VGABuffer,
}

#[allow(unused)]
impl VGAWriter {
    pub fn new() -> Self {
        VGAWriter {
            col: 0,
            color: ColorMode::new(TextColor::Green, TextColor::Black, false),
            buffer: VGABuffer::new(),
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        if byte == b'\n' || self.col == VGA_COLS {
            self.buffer.move_rows_up();
            self.col = 0;
            self.buffer.clear_row(VGA_ROWS - 1);
        }
        if byte == b'\n' {
            return;
        }
        let vga_byte = match byte {
            0x20..=0x7e => byte,
            // love emoji if not a valid ASCII value
            _ => 3,
        };

        self.buffer.write_byte(
            VGA_ROWS - 1,
            self.col,
            VGAChar {
                byte: vga_byte,
                color: self.color,
            },
        );
        self.col += 1;
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.write_byte(byte);
        }
    }

    pub fn write_string(&mut self, string: &str) {
        for byte in string.bytes() {
            self.write_byte(byte);
        }
    }
}

// Implement `fmt::write` trait for writing or formatting into our buffer
use core::fmt;
impl fmt::Write for VGAWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}
