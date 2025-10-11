#![no_std]
#![no_main]

use core::panic::PanicInfo;
use kalopsia_os::println;

// Custom panic handler since std lib is disabled
#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    println!("{_info}");
    loop {}
}

/** `_start` function
 * Custom entry point to overwite default rust's crt0 entry point
 * Disabled name mangling to help the linker.
 * use extern "C" so that the compiler uses C calling convention for this function
 */
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World!, ");
    println!("this is {}", "kalopsia-os");

    loop {}
}
