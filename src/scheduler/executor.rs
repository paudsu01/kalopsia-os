use crate::scheduler::Task;
use alloc::collections::VecDeque;
use core::task::Context;
use core::task::Poll;

// "Scheduler" for kalopsia-os
// Relies on rust futures and async model, based on `poll`
#[allow(dead_code)]
pub struct Executor {
    tasks: VecDeque<Task>,
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
            tasks: VecDeque::new(),
        }
    }

    pub fn add(&mut self, task: Task) {
        self.tasks.push_back(task);
    }

    pub fn run(&mut self) {
        let waker = dummy_waker();
        let mut cx = Context::from_waker(&waker);

        // The idea is use this as a global executor for polling all futures in the system until they are finished !!!
        while let Some(mut task) = self.tasks.pop_front() {
            // Run the `task` until it needs to `wait` (i.e Pending)
            match task.poll(&mut cx) {
                Poll::Ready(()) => {} // task done
                Poll::Pending => self.add(task),
            }
        }
    }
}

// From Phil Opp's blog: Dummy waker!
use core::task::RawWakerVTable;
use core::task::{RawWaker, Waker};

fn dummy_waker() -> Waker {
    unsafe { Waker::from_raw(dummy_raw_waker()) }
}

fn dummy_raw_waker() -> RawWaker {
    fn no_op(_: *const ()) {}
    fn clone(_: *const ()) -> RawWaker {
        dummy_raw_waker()
    }

    let vtable = &RawWakerVTable::new(clone, no_op, no_op, no_op);
    RawWaker::new(0 as *const (), vtable)
}
