#![no_std]
#![no_main]

use core::panic::PanicInfo;

// Custom panic handler since std lib is disabled
#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/** `_start` function
 * Custom entry point to overwite default rust's crt0 entry point
 * Disabled name mangling to help the linker.
 * use extern "C" so that the compiler uses C calling convention for this function
 */
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // VGA buffer start address
    let vga = 0xb8000 as *mut u8;

    let hello_world: &[u8] = "HELLO WORLD!".as_bytes();
    unsafe {
        for (i, &byte) in hello_world.iter().enumerate() {
            *(vga.offset(i as isize * 2)) = byte;
            *(vga.offset(i as isize * 2 + 1)) = 0xa_u8;
        }
    }

    loop {}
}
