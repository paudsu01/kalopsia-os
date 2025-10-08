#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum TextColor {
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

pub struct ColorMode(u8);

impl ColorMode {
    pub fn new(foreground: TextColor, background: TextColor, blink: bool) -> Self {
        let color_byte = ((background as u8) << 4) | (foreground as u8) | ((blink as u8) << 7);
        ColorMode(color_byte)
    }
}

#[repr(C)]
pub struct VGAChar {
    pub byte: u8,
    pub color: ColorMode,
}

pub struct VGABuffer {
    start_address: *mut VGAChar,
}

static VGA_ROWS: u16 = 25;
static VGA_COLS: u16 = 80;

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
    pub fn new() -> Self {
        VGABuffer {
            start_address: (0xb8000 as *mut VGAChar),
        }
    }

    fn get_ptr(&self, row: u16, col: u16) -> Option<*mut VGAChar> {
        if row >= VGA_ROWS || col >= VGA_COLS {
            None
        } else {
            let addr = unsafe {
                // Get ptr to required row
                let row_addr = self.start_address.offset((row * VGA_COLS) as isize);
                // Get ptr to required col offset in the row
                row_addr.offset(col as isize)
            };
            Some(addr)
        }
    }

    pub fn write_byte(&self, row: u16, col: u16, byte: VGAChar) {
        let ptr = self.get_ptr(row, col);
        let Some(ptr) = ptr else {
            return;
        };
        unsafe {
            *ptr = byte;
        }
    }
}
