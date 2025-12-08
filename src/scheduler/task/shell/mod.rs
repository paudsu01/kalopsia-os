use crate::scheduler::task::keyboard::get_line;
use crate::vga_buffer::{TextColor, VGA_WRITER};
use crate::{print, println};

mod subtasks;
use subtasks::CommandRegistry;

pub async fn shell() {
    // All supported 'commands'
    let subtasks: CommandRegistry = subtasks::load();
    // unix shell logic (fork -> exec -> wait) minus the fork
    loop {
        print_prompt();
        // Read the command
        let line = get_line().await; // async
        if !line.is_empty() {
            let (command, arguments) = parse_command(&line);
            // get the handler for the command
            let handler = subtasks.get_command_handler(command, arguments);
            match handler {
                None => {
                    println!("shell: command not found: {}", command);
                }
                Some(subtask) => {
                    // run the new task
                    // wait for it to be complete
                    subtask.await;
                }
            }
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

fn parse_command(input: &str) -> (&str, &str) {
    match input.trim().split_once(char::is_whitespace) {
        Some((cmd, args)) => (cmd, args.trim()),
        None => (input.trim(), ""),
    }
}
