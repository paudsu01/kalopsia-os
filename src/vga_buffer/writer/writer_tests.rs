use core::fmt::Write;

use super::VGA_WRITER;

#[test_case]
fn test_print_simple() {
    print!("Hello World!");
}

#[test_case]
fn test_println_simple() {
    println!("Hello World!");
}

#[test_case]
fn test_println_output() {
    use crate::interrupts;

    interrupts::without_interrupts(|| {
        let mut writer = VGA_WRITER.lock();
        let start_row = writer.row;
        let start_col = writer.col;

        let string = "Hello World!, This is kalopsia OS";
        let mut string_bytes = string.bytes();
        // Use write_str instead because `print` macro disables and enables interrupts back. We
        // want the whole test to be atomic
        writer.write_str(string).unwrap();

        for row in start_row..(writer.row + 1) {
            for col in start_col..writer.col {
                assert_eq!(
                    writer.buffer.read_char(row, col).unwrap(),
                    string_bytes.next().unwrap()
                );
            }
        }
    });
}
