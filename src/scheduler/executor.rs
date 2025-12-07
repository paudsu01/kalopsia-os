use crate::interrupts;
use crate::scheduler::task::{TaskId, TaskWaker};
use crate::scheduler::Task;
use alloc::collections::btree_map::BTreeMap;
use alloc::sync::Arc;
use core::task::Context;
use core::task::Poll;
use crossbeam_queue::ArrayQueue;

// "Scheduler" for kalopsia-os
// Relies on rust futures and async model, based on `poll`
pub struct Executor {
    // tasks to poll
    tasks_queue: Arc<ArrayQueue<TaskId>>,
    // all the tasks
    tasks: BTreeMap<TaskId, Task>,
}

#[allow(dead_code)]
impl Default for Executor {
    fn default() -> Self {
        Self::new()
    }
}

impl Executor {
    pub fn new() -> Executor {
        Executor {
            tasks: BTreeMap::new(),
            tasks_queue: Arc::new(ArrayQueue::new(200)),
        }
    }

    pub fn spawn_task(&mut self, task: Task) {
        let id = task.id;
        self.tasks.insert(id, task);
        self.queue_task(id);
    }

    pub fn queue_task(&mut self, id: TaskId) {
        self.tasks_queue.push(id).expect("Tasks queue is full");
    }

    pub fn run(&mut self) -> ! {
        // The idea is use this as a global executor for polling all futures in the system until they are finished !!!
        loop {
            self.run_ready_tasks();
            self.sleep();
        }
    }

    pub fn sleep(&mut self) {
        // Task could be `woken` up at this point(after this function was called)
        // So, we verify by disabling interrupts and checking if tasks queue is empty or not!
        interrupts::disable();
        if self.tasks_queue.is_empty() {
            x86_64::instructions::interrupts::enable_and_hlt();
        } else {
            interrupts::enable();
        }
    }

    fn run_ready_tasks(&mut self) {
        while let Some(task_id) = self.tasks_queue.pop() {
            let waker = TaskWaker::new_waker(task_id, Arc::clone(&self.tasks_queue));
            let mut cx = Context::from_waker(&waker);
            // Run the `task` until it needs to `wait` (i.e Pending)
            let task = self
                .tasks
                .get_mut(&task_id)
                .expect("Task should be present");
            match task.poll(&mut cx) {
                Poll::Ready(()) => {} // task done
                Poll::Pending => {}   // do nothing (waker will add it back)
            }
        }
    }
}
