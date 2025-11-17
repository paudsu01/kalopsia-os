#![no_std]
#![no_main]

use core::panic::PanicInfo;
use kalopsia_os::println;

// Custom panic handler since std lib is disabled
#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    println!("{_info}");
    kalopsia_os::hlt();
}

/** `_start` function
 * Custom entry point to overwite default rust's crt0 entry point
 * Disabled name mangling to help the linker.
 * use extern "C" so that the compiler uses C calling convention for this function
 */
#[unsafe(no_mangle)]
pub extern "C" fn _start(boot_info: &'static bootloader::BootInfo) -> ! {
    kalopsia_os::init(boot_info.physical_memory_offset);
    let lvl_4_page_table = kalopsia_os::memory::active_lvl_4_pt();

    println!("Hello World!, ");
    println!("this is {}", "kalopsia-os");
    for (i, pte) in lvl_4_page_table.iter().enumerate() {
        if !pte.is_unused() {
            println!("Entry {}: Value: {:?}", i, pte);
        }
    }

    kalopsia_os::hlt();
}
