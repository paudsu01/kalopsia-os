use crate::interrupts::{Interrupts, PICS};
use crate::print;
use crate::utils::Port;

use lazy_static::lazy_static;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use spin::Mutex;
use x86_64::structures::idt::InterruptStackFrame;

lazy_static! {
    pub static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new({
        Keyboard::new(
            ScancodeSet1::new(),
            layouts::Us104Key,
            HandleControl::Ignore,
        )
    });
}

pub extern "x86-interrupt" fn keyboard_interrupt_handler(_frame: InterruptStackFrame) {
    let mut port = Port::new(0x60);
    let scancode: u8 = port.readb();

    let mut keyboard = KEYBOARD.lock();
    if let Ok(op_keyevent) = keyboard.add_byte(scancode) && let Some(keyevent) = op_keyevent && let Some(keyvalue) = keyboard.process_keyevent(keyevent) {
        match keyvalue {
            DecodedKey::Unicode(character) => print!("{}", character),
            DecodedKey::RawKey(key) => print!("{:?}", key),
        }
    }

    PICS.lock().end_of_interrupt(Interrupts::Keyboard as u8);
}
