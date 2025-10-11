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
    let start_row: u16;
    let start_col: u16;
    {
        let writer = VGA_WRITER.lock();
        start_row = writer.row;
        start_col = writer.col;
    }

    let string = "Hello World!, This is kalopsia OS";
    let mut string_bytes = string.bytes();
    print!("{string}");

    let writer = VGA_WRITER.lock();
    for row in start_row..(writer.row + 1) {
        for col in start_col..writer.col {
            assert_eq!(
                writer.buffer.read_char(row, col).unwrap(),
                string_bytes.next().unwrap()
            );
        }
    }
}
