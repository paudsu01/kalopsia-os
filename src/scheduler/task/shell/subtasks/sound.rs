use crate::utils::Port;
use alloc::string::String;

// Credit: OSDev
pub async fn beep(_args: String) {
    let div: u32 = 1193180 / 1000;
    // Setup ports
    let mut p43 = Port::new(0x43);
    let mut p42 = Port::new(0x42);
    let mut p61 = Port::new(0x61);

    p43.writeb(0xb6);
    p42.writeb(div as u8);
    p42.writeb((div >> 8) as u8);

    // Turn the speaker on
    if _args.is_empty() || _args == "on" {
        let tmp = p61.readb();
        if tmp != (tmp | 3) {
            p61.writeb(tmp | 3);
        }
    } else {
        // Turn Speaker OFF
        let tmp = p61.readb();
        p61.writeb(tmp & 0b11111100); // Clear lowest 2 bits
    }
}
