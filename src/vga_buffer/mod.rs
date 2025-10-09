#[allow(dead_code)]
#[derive(Clone, Copy)]
enum TextColor {
    Black = 0x0,
    Blue = 0x1,
    Green = 0x2,
    Cyan = 0x3,
    Red = 0x4,
    Magenta = 0x5,
    Brown = 0x6,
    LightGray = 0x7,
    DarkGray = 0x8,
    LightBlue = 0x9,
    LightGreen = 0xa,
    LightCyan = 0xb,
    LightRed = 0xc,
    Pink = 0xd,
    Yellow = 0xe,
    White = 0xf,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct ColorMode(u8);

impl ColorMode {
    fn new(foreground: TextColor, background: TextColor, blink: bool) -> Self {
        let color_byte = ((background as u8) << 4) | (foreground as u8) | ((blink as u8) << 7);
        ColorMode(color_byte)
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
struct VGAChar {
    byte: u8,
    color: ColorMode,
}

struct VGABuffer;

const VGA_ROWS: u16 = 25;
const VGA_COLS: u16 = 80;

/*  The VGA buffer starts at addr 0xb8000
   The VGA buffer is a 2d-array with 25 rows and 80 columns
   Each element is 16 bit long such that
       Bit 0-7: ASCII code print
       Bit 8-11: Foreground color
       Bit 12-14: Background color
       Bit 15: Blink
   For more details: https://wiki.osdev.org/Text_UI
*/
impl VGABuffer {
    fn get_ptr(&self, row: u16, col: u16) -> Option<*mut VGAChar> {
        if row >= VGA_ROWS || col >= VGA_COLS {
            None
        } else {
            let addr = unsafe {
                // Get ptr to required row
                let row_addr = (0xb8000 as *mut VGAChar).offset((row * VGA_COLS) as isize);
                // Get ptr to required col offset in the row
                row_addr.offset(col as isize)
            };
            Some(addr)
        }
    }

    fn write_byte(&self, row: u16, col: u16, byte: VGAChar) -> bool {
        let ptr = self.get_ptr(row, col);
        let Some(ptr) = ptr else {
            return false;
        };
        unsafe {
            // write volatile guarantees that the write isn't optimized away
            // which can happen because the compiler **thinks** the writes are unnecessary (because
            // we don't access the modified memory)
            core::ptr::write_volatile(ptr, byte);
        }
        true
    }

    fn move_rows_up(&self) {
        for row in 1..VGA_ROWS {
            for col in 0..VGA_COLS {
                // `unwrap` will never fail here
                let ptr = self.get_ptr(row, col).unwrap();
                unsafe {
                    self.write_byte(row - 1, col, *ptr);
                }
            }
        }
    }

    fn clear_row(&self, row: u16) {
        for col in 0..VGA_COLS {
            self.write_byte(
                row,
                col,
                VGAChar {
                    byte: b' ',
                    color: ColorMode::new(TextColor::Black, TextColor::Black, false),
                },
            );
        }
    }
}

mod writer;
pub use writer::_print;
