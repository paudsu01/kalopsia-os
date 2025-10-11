#![no_std]
#![no_main]
// custom test framework for `cargo test`
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_framework::custom_test_runner)]
#![reexport_test_harness_main = "testing_start"]

use core::panic::PanicInfo;

// Custom user defined runner function for the custom test framework
#[cfg(test)]
mod test_framework;
// VGA buffer mode
mod vga_buffer;

// Custom panic handler since std lib is disabled
#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    #[cfg(test)]
    {
        serial_println!("[Fail]");
        serial_println!("Error: {}", _info);
        test_framework::exit_qemu(test_framework::QEMUExitCode::Failure);
    }

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

    #[cfg(test)]
    testing_start();

    loop {}
}
