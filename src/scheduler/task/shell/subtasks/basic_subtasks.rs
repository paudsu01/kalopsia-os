use alloc::string::String;

use crate::vga_buffer::VGA_WRITER;

pub async fn echo(args: String) {
    crate::println!("{}", args);
}

pub async fn clear(_args: String) {
    VGA_WRITER.lock().clear_screen();
}
