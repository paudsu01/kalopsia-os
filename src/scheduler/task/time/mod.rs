use crate::{datetime_print, utils::Port};
use core::task::Poll;
use futures_util::task::AtomicWaker;

pub static DATETIME_WAKER: AtomicWaker = AtomicWaker::new();

struct DateTime;

// The idea is that this is a never ending task
// After each poll, the waker will `wake` this task during the next interrupt
// which will update the time
impl Future for DateTime {
    type Output = ();
    fn poll(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> Poll<()> {
        let mut status_port = Port::new(0x70);
        let mut data_port = Port::new(0x71);

        // Wait for update to finish
        // TODO: Maybe possible to poll here and use this as a future but i am not sure
        // how the waker would work/ be placed
        loop {
            status_port.writeb(0x0A); // status register A
            let a = data_port.readb();
            if a & 0x80 == 0 {
                break;
            } // bit 7 = update-in-progress
        }

        datetime_print!(
            "Current time: {}:{}:{} {}/{}/{}",
            bcd_to_bin(read_rtc(0x04)), // hour
            bcd_to_bin(read_rtc(0x02)), // minute
            bcd_to_bin(read_rtc(0x00)), // seconds
            bcd_to_bin(read_rtc(0x07)), // day of month
            bcd_to_bin(read_rtc(0x08)), // month
            bcd_to_bin(read_rtc(0x09)), // last two digits of year
        );
        // register waker
        DATETIME_WAKER.register(cx.waker());
        Poll::Pending
    }
}

// This task will never end
// No need for async fn here, we could have just used the `DateTime` struct directly
// Kept the async fn for keeping things same for tasks specification
pub async fn current_time() {
    DateTime.await;
}

fn read_rtc(register: u8) -> u8 {
    let mut index_port = Port::new(0x70); // need to specify which register to read from
    index_port.writeb(register);
    let mut data_port = Port::new(0x71); // This port gives us what the register value is
    data_port.readb()
}

// BCD to binary
fn bcd_to_bin(bcd: u8) -> u8 {
    (bcd & 0x0F) + ((bcd >> 4) * 10)
}
