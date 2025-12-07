use crate::println;
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

    pub fn push(&self, value: u8) {
        if self.queue.push(value).is_err() {
            println!("Scancode queue is full! Dropping input");
        }
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
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<u8> {
        match SCANCODE_QUEUE.pop() {
            None => Poll::Pending,
            Some(v) => Poll::Ready(v),
        }
    }
}
