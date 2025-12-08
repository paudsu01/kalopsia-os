use crate::scheduler::task::keyboard::get_line;
use crate::vga_buffer::{TextColor, VGA_WRITER};
use crate::{print, println};

pub async fn shell() {
    // unix shell logic (fork -> exec -> wait) minus the fork
    loop {
        print_prompt();
        // Read the command
        let word = get_line().await;
        // Store a command name to function pointer
        // Call the function with required arguments
        // Wait for the future to resolve
        if !word.is_empty() {
            println!("{}", word)
        };
    }
}

fn print_prompt() {
    VGA_WRITER.lock().change_text_color(TextColor::Yellow);
    print!("admin");
    VGA_WRITER.lock().change_text_color(TextColor::Cyan);
    print!("@kalopsia-os");
    VGA_WRITER.lock().change_text_color(TextColor::White);
    print!(" $ ");
}
