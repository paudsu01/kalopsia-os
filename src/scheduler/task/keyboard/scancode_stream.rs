use crate::scheduler::task::keyboard::KEYBOARD_WAKER;
use core::{
    pin::Pin,
    task::{Context, Poll},
};
use crossbeam_queue::ArrayQueue;
use lazy_static::lazy_static;

const SCANCODE_QUEUE_SIZE: usize = 100;

pub struct ScancodeQueue {
    pub queue: ArrayQueue<u8>,
}

lazy_static! {
    // We need to make sure
    pub static ref SCANCODE_QUEUE: ScancodeQueue = ScancodeQueue::new(SCANCODE_QUEUE_SIZE);
}

#[allow(dead_code)]
impl ScancodeQueue {
    pub fn new(size: usize) -> Self {
        ScancodeQueue {
            queue: ArrayQueue::new(size),
        }
    }

    pub fn push(&self, value: u8) -> Result<(), u8> {
        self.queue.push(value)
    }

    pub fn pop(&self) -> Option<u8> {
        self.queue.pop()
    }

    pub fn next(&self) -> Scancode {
        Scancode
    }
}

pub fn init() {
    // Force evaluation of lazy_static
    lazy_static::initialize(&SCANCODE_QUEUE);
}

pub struct Scancode;

impl Future for Scancode {
    type Output = u8;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<u8> {
        // fast path: scancode already available (queue not empty)
        if let Some(value) = SCANCODE_QUEUE.pop() {
            return Poll::Ready(value);
        }

        // queue **potentially** empty (interrupt might have added something to the queue)
        KEYBOARD_WAKER.register(cx.waker());
        match SCANCODE_QUEUE.pop() {
            // Keyboard interrupt occured and added something to the queue
            Some(value) => {
                KEYBOARD_WAKER.take();
                Poll::Ready(value)
            }
            None => Poll::Pending,
        }
    }
}
