use alloc::boxed::Box;
use core::future::Future;
use core::pin::Pin;
use core::sync::atomic::{AtomicU64, Ordering};
use core::task::{Context, Poll};

static TASK_ID: AtomicU64 = AtomicU64::new(0);

pub mod keyboard;
pub mod time;

#[allow(dead_code)]
pub struct Task {
    id: u64,
    future: Pin<Box<dyn Future<Output = ()>>>,
}

#[allow(dead_code)]
impl Task {
    pub fn new<T>(future: T) -> Self
    where
        T: Future<Output = ()> + 'static,
    {
        Task {
            id: TASK_ID.fetch_add(1, Ordering::Relaxed),
            future: Box::pin(future),
        }
    }

    pub fn poll(&mut self, cx: &mut Context<'_>) -> Poll<()> {
        self.future.as_mut().poll(cx)
    }
}
