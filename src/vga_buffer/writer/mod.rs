use crate::vga_buffer::VGA_ROWS;

use super::{ColorMode, TextColor, VGABuffer, VGAChar, VGA_COLS};

#[allow(unused)]
struct VGAWriter {
    row: u16,
    col: u16,
    color: ColorMode,
    buffer: VGABuffer,
}

#[allow(unused)]
impl VGAWriter {
    fn new() -> Self {
        VGAWriter {
            row: 0,
            col: 0,
            color: ColorMode::new(TextColor::Green, TextColor::Black, false),
            buffer: VGABuffer,
        }
    }

    fn write_byte(&mut self, byte: u8) {
        if byte == b'\n' || self.col == VGA_COLS {
            if self.row == VGA_ROWS - 1 {
                self.buffer.move_rows_up();
                self.buffer.clear_row(VGA_ROWS - 1);
            } else {
                self.row += 1;
            };
            self.col = 0;
        }
        if byte == b'\n' {
            return;
        }
        let vga_byte = match byte {
            0x20..=0x7e => byte,
            // love emoji if not a valid ASCII value
            _ => 3,
        };

        self.buffer.write_char(
            self.row,
            self.col,
            VGAChar {
                byte: vga_byte,
                color: self.color,
            },
        );
        self.col += 1;
    }

    fn write_bytes(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.write_byte(byte);
        }
    }

    fn write_string(&mut self, string: &str) {
        for byte in string.bytes() {
            self.write_byte(byte);
        }
    }
}

use core::fmt;
// Implement `fmt::write` trait for writing or formatting into our buffer
impl fmt::Write for VGAWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

// lazy initialization of VGA_WRITER because statics require a const initializer, and VGAWriter::new() is a non-const function.
// We wrap it in a spin::Mutex to provide safe interior mutability in a single-threaded or
// multi-threaded context
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    static ref VGA_WRITER: Mutex<VGAWriter> = Mutex::new(VGAWriter::new());
}
// Macro defs for printing stuff to the screen
// Used phil opp's macro defs: https://os.phil-opp.com/vga-text-mode/#a-println-macro
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    VGA_WRITER.lock().write_fmt(args).unwrap();
}

#[cfg(test)]
mod writer_tests;
