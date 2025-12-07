use crate::println;
use crate::utils::Port;

#[derive(Debug)]
pub struct Time {
    hour: u8,
    minute: u8,
    second: u8,
    day: u8,
    month: u8,
    year: u8,
}

fn read_rtc(register: u8) -> u8 {
    let mut index_port = Port::new(0x70);
    let mut data_port = Port::new(0x71);
    index_port.writeb(register);
    data_port.readb()
}

// BCD to binary
fn bcd_to_bin(bcd: u8) -> u8 {
    (bcd & 0x0F) + ((bcd >> 4) * 10)
}

pub async fn read_time() {
    // Wait for update to finish
    let mut status_port = Port::new(0x70);
    let mut data_port = Port::new(0x71);

    loop {
        status_port.writeb(0x0A); // status register A
        let a = data_port.readb();
        if a & 0x80 == 0 {
            break;
        } // bit 7 = update-in-progress
    }

    let time = Time {
        second: bcd_to_bin(read_rtc(0x00)),
        minute: bcd_to_bin(read_rtc(0x02)),
        hour: bcd_to_bin(read_rtc(0x04)),
        day: bcd_to_bin(read_rtc(0x07)),
        month: bcd_to_bin(read_rtc(0x08)),
        year: bcd_to_bin(read_rtc(0x09)),
    };

    println!("Current time: {:?}:", time);
}
