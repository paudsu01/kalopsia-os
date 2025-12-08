use super::super::{ColorMode, TextColor, VGABuffer, VGAChar, VGA_COLS};
use super::VGAByteWriter;
use crate::vga_buffer::VGA_ROWS;

struct TimerWriter {
    // will always occupy the last row (VGA_ROWS -1)
    col: u16,
    color: ColorMode,
    buffer: VGABuffer,
}

impl VGAByteWriter for TimerWriter {
    fn write_byte(&mut self, byte: u8) {
        if self.col == VGA_COLS {
            self.buffer.clear_row(VGA_ROWS - 1);
            self.col = 0;
        }

        self.buffer.write_char(
            VGA_ROWS - 1,
            self.col,
            VGAChar {
                byte,
                color: self.color,
            },
        );
        self.col += 1;
    }
}

impl TimerWriter {
    fn new() -> Self {
        TimerWriter {
            col: 0,
            color: ColorMode::new(TextColor::Yellow, TextColor::Black, false),
            buffer: VGABuffer,
        }
    }
}

// Macro setup
use core::fmt;
// Implement `fmt::write` trait for writing or formatting into our buffer
impl fmt::Write for TimerWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    static ref TIMER_WRITER: Mutex<TimerWriter> = Mutex::new(TimerWriter::new());
}

#[macro_export]
macro_rules! timer_print {
    ($($arg:tt)*) => ($crate::vga_buffer::_timer_print(format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _timer_print(args: fmt::Arguments) {
    use core::fmt::Write;
    TIMER_WRITER.lock().write_fmt(args).unwrap();
}
