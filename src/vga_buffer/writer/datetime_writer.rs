// Abstraction that controls the second last row of the VGA Buffer
// Not the ideal way to do it but its fine for now
use super::super::{ColorMode, TextColor, VGABuffer, VGAChar};
use super::VGAByteWriter;
use crate::vga_buffer::VGA_ROWS;

#[allow(unused)]
pub struct DateTimerWriter {
    // will always occupy the last row (VGA_ROWS -1)
    col: u16,
    color: ColorMode,
    buffer: VGABuffer,
}

impl VGAByteWriter for DateTimerWriter {
    fn write_byte(&mut self, byte: u8) {
        self.buffer.write_char(
            VGA_ROWS - 2,
            self.col,
            VGAChar {
                byte,
                color: self.color,
            },
        );
        self.col += 1;
    }
}

#[allow(unused)]
impl DateTimerWriter {
    fn new() -> Self {
        DateTimerWriter {
            col: 0,
            color: ColorMode::new(TextColor::Yellow, TextColor::Black, false),
            buffer: VGABuffer,
        }
    }
}

use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    static ref DATETIME_WRITER: Mutex<DateTimerWriter> = Mutex::new(DateTimerWriter::new());
}

// Macro setup
use core::fmt;
// Implement `fmt::write` trait for writing or formatting into our buffer
impl fmt::Write for DateTimerWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! datetime_print {
    ($($arg:tt)*) => ($crate::vga_buffer::_datetime_print(format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _datetime_print(args: fmt::Arguments) {
    use core::fmt::Write;
    let mut datetime = DATETIME_WRITER.lock();
    datetime.buffer.clear_row(VGA_ROWS - 2);
    datetime.write_fmt(args).unwrap();
    datetime.col = 0;
}
