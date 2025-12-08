use crate::scheduler::Task;
use alloc::boxed::Box;
use alloc::collections::btree_map::BTreeMap;
use alloc::string::{String, ToString};
use core::pin::Pin;
use core::task::{Context, Poll};

mod basic_subtasks;

pub struct SubTask(Task);
impl Future for SubTask {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.0.poll(cx)
    }
}

type CommandHandler = Box<dyn Fn(String) -> SubTask>;
pub struct CommandRegistry {
    // mapping from command name to the fn handler
    // the fn handler when called returns the subtask (future)
    map: BTreeMap<String, CommandHandler>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }

    pub fn register(&mut self, name: &str, handler: CommandHandler) {
        self.map.insert(name.to_string(), handler);
    }

    // This creates the future by calling the stored function
    pub fn get_command_handler(&self, command_name: &str, args: &str) -> Option<SubTask> {
        self.map
            .get(command_name)
            .map(|handler| handler(args.to_string()))
    }
}

// All built-in commands should be loaded here
// TODO: once virtual filesystem need a notion of the path env variable to search for commands
pub fn load() -> CommandRegistry {
    let mut registry = CommandRegistry::new();

    registry.register(
        "echo",
        Box::new(|args| SubTask(Task::new(basic_subtasks::echo(args)))),
    );

    registry
}
