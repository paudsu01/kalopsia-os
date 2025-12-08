use crate::{
    interrupts::{KEYBOARD_INTERRUPTS_COUNT, TIMER_INTERRUPTS_COUNT},
    print, println,
    scheduler::task::keyboard::{convert_scancode, scancode_stream::SCANCODE_QUEUE}, vga_buffer::VGA_WRITER,
};
use core::pin::Pin;
use core::sync::atomic::Ordering;
use core::task::{Context, Poll};
use futures_util::task::AtomicWaker;

pub static INTERRUPT_COUNTER_WAKER: AtomicWaker = AtomicWaker::new();

pub struct InterruptCounter;

// The idea is that this is a never ending task
// After each poll, the waker will `wake` this task during the next interrupt
// which will update the time
impl Future for InterruptCounter {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        // register the waker so we wake up during a timer interrupt if the process is active
        INTERRUPT_COUNTER_WAKER.register(cx.waker());

        loop {
            // loop until the queue returns Poll::Pending (Empty)
            let scancode = SCANCODE_QUEUE.pop();
            match scancode {
                None => break,
                Some(scancode) => {
                    if let Some(char) = convert_scancode(scancode) && char == 'q' {
                            VGA_WRITER.lock().escape_print = false;
                            println!("\nQuitting Interrupt Counter.");
                            return Poll::Ready(());
                    }
                }
            }
        }
        print!(
            "Timer: {:<5} | Kbd: {:<5}",
            TIMER_INTERRUPTS_COUNT.load(Ordering::Relaxed),
            KEYBOARD_INTERRUPTS_COUNT.load(Ordering::Relaxed),
        );

        Poll::Pending
    }
}

// This task will end when user presses 'q'
// Kept the async fn for keeping things same for tasks specification
pub async fn int_count_command() {
    println!("Press `q'` to exit");
    VGA_WRITER.lock().escape_print = true;
    TIMER_INTERRUPTS_COUNT.store(0, Ordering::Relaxed);
    KEYBOARD_INTERRUPTS_COUNT.store(0, Ordering::Relaxed);
    InterruptCounter.await;
}
