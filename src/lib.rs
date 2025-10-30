#![no_std]
// Setup for running tests since cargo test will need to compile lib.rs to a test runner binary!
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(test_framework::custom_test_runner)]
#![reexport_test_harness_main = "test_main"]
// For the "x86-interrupt" calling convention which is unstable
#![feature(abi_x86_interrupt)]

pub mod gdt;
pub mod interrupts;
pub mod test_framework;
pub mod utils;
pub mod vga_buffer;

use core::panic::PanicInfo;
pub use test_framework::{exit_qemu, QEMUExitCode};

pub fn init() {
    gdt::init();
    interrupts::init_idt(); // Load the IDT
    interrupts::init_pics(); // Init PIC with new offsets so that interrupt numbers don't overlap
                             // exception indexes in the IDT
    interrupts::enable(); // Enable interrupt with the `sti` instruction
}

pub fn hlt() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

// Entry point for `cargo test`
#[cfg(test)]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    init();
    test_main();
    hlt();
}

pub fn panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QEMUExitCode::Failure);
    hlt();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    panic_handler(info);
}
