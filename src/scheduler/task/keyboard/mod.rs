use crate::print;
use crate::scheduler::task::keyboard::scancode_stream::SCANCODE_QUEUE;
use alloc::string::String;
use lazy_static::lazy_static;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use futures_util::task::AtomicWaker;
use spin::Mutex;

pub mod scancode_stream;

lazy_static! {
    pub static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new({
        Keyboard::new(
            ScancodeSet1::new(),
            layouts::Us104Key,
            HandleControl::Ignore,
        )
    });
}


pub static KEYBOARD_WAKER: AtomicWaker = AtomicWaker::new();

pub async fn get_line() -> String{ 
    let mut keyboard = KEYBOARD.lock();
    let mut word = String::new();
    // keep adding scancodes until we get a `\n` character
    loop {
        let scancode = SCANCODE_QUEUE.next().await;
        // process the scancode to get the char
        if let Ok(op_keyevent) = keyboard.add_byte(scancode) && let Some(keyevent) = op_keyevent && let Some(keyvalue) = keyboard.process_keyevent(keyevent) {
            let character = if let DecodedKey::Unicode(character) = keyvalue {
                character
            } else {
                '\u{2764}'
            };
            
            print!("{}", character);
            // return word if `\n`
            if character != '\n'{
                word.push(character);
            } else{
                return word;
            }
        }
    }
}
