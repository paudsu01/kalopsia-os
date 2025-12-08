use alloc::string::String;

use crate::vga_buffer::VGA_WRITER;

pub async fn echo(args: String) {
    crate::println!("{}", args);
}

pub async fn clear() {
    VGA_WRITER.lock().clear_screen();
}
