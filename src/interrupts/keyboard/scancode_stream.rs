use crossbeam_queue::ArrayQueue;
use lazy_static::lazy_static;

const SCANCODE_QUEUE_SIZE: usize = 100;

pub struct ScancodeStream {
    queue: ArrayQueue<u8>,
}

lazy_static! {
    // We need to make sure
    pub static ref SCANCODE_STREAM: ScancodeStream = ScancodeStream::new(SCANCODE_QUEUE_SIZE);
}

#[allow(dead_code)]
impl ScancodeStream {
    pub fn new(size: usize) -> Self {
        ScancodeStream {
            queue: ArrayQueue::new(size),
        }
    }

    pub fn push(&self, value: u8) {
        self.queue.push(value).expect("Scancode queue is full");
    }

    pub fn pop(&self) -> Option<u8> {
        self.queue.pop()
    }
}

pub fn init() {
    // Force evaluation of lazy_static
    lazy_static::initialize(&SCANCODE_STREAM);
}
