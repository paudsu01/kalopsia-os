use crate::scheduler::Task;
use alloc::boxed::Box;
use alloc::collections::btree_map::BTreeMap;
use alloc::string::{String, ToString};
use core::pin::Pin;
use core::task::{Context, Poll};

mod ascii_print;
mod interrupt_counter;
pub use interrupt_counter::INTERRUPT_COUNTER_WAKER;
mod memdump;
pub use ascii_print::pretty_echo_command;
mod basic_subtasks;
mod sound;

pub struct SubTask(Task);
impl Future for SubTask {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.0.poll(cx)
    }
}

type CommandHandler = Box<dyn Fn(String) -> SubTask>;
type CommandInfo = (&'static str, CommandHandler);
pub struct CommandRegistry {
    // mapping from command name to the fn handler
    // the fn handler when called returns the subtask (future)
    pub map: BTreeMap<String, CommandInfo>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }

    pub fn register(&mut self, name: &str, help_message: &'static str, handler: CommandHandler) {
        self.map.insert(name.to_string(), (help_message, handler));
    }

    // This creates the future by calling the stored function
    pub fn get_command_handler(&self, command_name: &str, args: &str) -> Option<SubTask> {
        let command_info = self.map.get(command_name);
        command_info.map(|command_info| {
            let handler = &command_info.1;
            handler(args.to_string())
        })
    }
}

// All built-in commands should be loaded here
// TODO: once virtual filesystem need a notion of the path env variable to search for commands
pub fn load() -> CommandRegistry {
    let mut registry = CommandRegistry::new();

    registry.register(
        "echo",
        "prints to the screen",
        Box::new(|args| SubTask(Task::new(basic_subtasks::echo(args)))),
    );

    registry.register(
        "clear",
        "clear the screen",
        Box::new(|_args| SubTask(Task::new(basic_subtasks::clear()))),
    );

    registry.register(
        "beep",
        "`on` plays sound. `off` or anything else turns it off",
        Box::new(|args| SubTask(Task::new(sound::beep(args)))),
    );

    registry.register(
        "pecho",
        "pretty version of echo",
        Box::new(|args| SubTask(Task::new(ascii_print::pretty_echo_command(args)))),
    );

    registry.register(
        "memdump",
        "Dumps memory(can crash!) Usage: memdump addr bytes",
        Box::new(|args| SubTask(Task::new(memdump::memdump_command(args)))),
    );

    registry.register(
        "int-count",
        "count the number of interrupts in the system. Press `q` to exit",
        Box::new(|_args| SubTask(Task::new(interrupt_counter::int_count_command()))),
    );

    registry
}
