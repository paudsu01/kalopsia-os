use super::super::{ColorMode, TextColor, VGABuffer, VGAChar, VGA_COLS};
use super::VGAByteWriter;
use crate::vga_buffer::VGA_ROWS;

pub struct VGAWriter {
    row: u16,
    col: u16,
    color: ColorMode,
    buffer: VGABuffer,
    pub escape_print: bool, // if on, it will act like every string to print has a '\r' in front, clears
                            // the current row and print
}

impl VGAByteWriter for VGAWriter {
    fn write_byte(&mut self, byte: u8) {
        if byte == b'\n' || self.col == VGA_COLS {
            if self.row == VGA_ROWS - 3 {
                self.buffer.move_rows_up(VGA_ROWS - 2);
                self.buffer.clear_row(VGA_ROWS - 3);
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
}

impl VGAWriter {
    fn new() -> Self {
        VGAWriter {
            row: 0,
            col: 0,
            color: ColorMode::new(TextColor::White, TextColor::Black, false),
            buffer: VGABuffer,
            escape_print: false,
        }
    }

    pub fn change_text_color(&mut self, color: TextColor) {
        self.color = ColorMode::new(color, TextColor::Black, false);
    }

    pub fn clear_screen(&mut self) {
        for row in 0..VGA_ROWS - 2 {
            self.buffer.clear_row(row);
        }
        self.row = 0;
        self.col = 0;
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
    pub static ref VGA_WRITER: Mutex<VGAWriter> = Mutex::new(VGAWriter::new());
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
    use crate::interrupts;
    use core::fmt::Write;

    interrupts::without_interrupts(|| {
        let mut writer = VGA_WRITER.lock();
        if writer.escape_print {
            writer.buffer.clear_row(writer.row);
            writer.col = 0;
        }
        writer.write_fmt(args).unwrap();
    });
}

#[cfg(test)]
mod writer_tests;
