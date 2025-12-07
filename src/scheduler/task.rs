use alloc::boxed::Box;
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};

#[allow(dead_code)]
pub struct Task {
    future: Pin<Box<dyn Future<Output = ()>>>,
}

#[allow(dead_code)]
impl Task {
    pub fn new<T>(future: T) -> Self
    where
        T: Future<Output = ()> + 'static,
    {
        Task {
            future: Box::pin(future),
        }
    }
}

impl Future for Task {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        self.future.as_mut().poll(cx)
    }
}
