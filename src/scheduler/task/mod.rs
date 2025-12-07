use alloc::boxed::Box;
use alloc::sync::Arc;
use alloc::task::Wake;
use core::future::Future;
use core::pin::Pin;
use core::sync::atomic::{AtomicU64, Ordering};
use core::task::{Context, Poll, Waker};
use crossbeam_queue::ArrayQueue;

static TASK_ID: AtomicU64 = AtomicU64::new(0);

pub mod keyboard;
pub mod time;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TaskId(u64);

#[allow(dead_code)]
pub struct Task {
    pub id: TaskId,
    future: Pin<Box<dyn Future<Output = ()>>>,
}

impl Task {
    pub fn new<T>(future: T) -> Self
    where
        T: Future<Output = ()> + 'static,
    {
        Task {
            id: TaskId(TASK_ID.fetch_add(1, Ordering::Relaxed)),
            future: Box::pin(future),
        }
    }

    pub fn poll(&mut self, cx: &mut Context<'_>) -> Poll<()> {
        self.future.as_mut().poll(cx)
    }
}

pub struct TaskWaker {
    task_id: TaskId,
    tasks: Arc<ArrayQueue<TaskId>>,
}

#[allow(dead_code)]
impl TaskWaker {
    pub fn new_waker(task_id: TaskId, tasks: Arc<ArrayQueue<TaskId>>) -> Waker {
        Waker::from(Arc::new(TaskWaker { task_id, tasks }))
    }
}

// The main idea is that when a executor polls a future and that returns Poll::Pending, we don't poll that future again and the executor removes it from the scheduling queue
// So, when a future is ready to be polled again(e.g keyboard interrupt for some keyboard related task), the waker is going to `wake` up the future
// by appending that future to the schedule queue so that it gets polled!!
impl Wake for TaskWaker {
    fn wake(self: Arc<Self>) {
        // Add it to the tasks queue so that the executor can poll the future that was `woke` up
        self.tasks.push(self.task_id).expect("Tasks is full");
    }
}
