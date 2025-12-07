#![no_std]
// Setup for running tests since cargo test will need to compile lib.rs to a test runner binary!
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(test_framework::custom_test_runner)]
#![reexport_test_harness_main = "test_main"]
// For the "x86-interrupt" calling convention which is unstable
#![feature(abi_x86_interrupt)]

// Add dependency to the built-in `alloc` crate
extern crate alloc;

pub mod gdt;
pub mod interrupts;
pub mod memory;
pub mod scheduler;
pub mod test_framework;
pub mod utils;
pub mod vga_buffer;

use core::panic::PanicInfo;
pub use test_framework::{exit_qemu, QEMUExitCode};
use x86_64::structures::paging::{FrameAllocator, Size4KiB};

pub fn init(physical_memory_offset: u64, frame_allocator: &mut impl FrameAllocator<Size4KiB>) {
    gdt::init();
    memory::init(physical_memory_offset); // Init memory to use offset for address translations
                                          // Init the heap
    memory::init_heap(frame_allocator).expect("Init: Heap init failed");
    drivers_init();

    interrupts::init_idt(); // Load the IDT
    interrupts::init_pics(); // Init PIC with new offsets so that interrupt numbers don't overlap
                             // exception indexes in the IDT
    interrupts::enable(); // Enable interrupt with the `sti` instruction
}

// Init keyboard scancode queue for now but want to expand this to init all drivers
pub fn drivers_init() {
    use interrupts::keyboard::scancode_stream;
    scancode_stream::init();
}

pub fn hlt() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

// Entry point for `cargo test`
#[cfg(test)]
#[unsafe(no_mangle)]
pub extern "C" fn _start(boot_info: &'static bootloader::BootInfo) -> ! {
    let mut frame_allocator = unsafe {
        memory::BootInfoFrameAllocator::init(memory::usable_frames(&boot_info.memory_map))
    };
    init(boot_info.physical_memory_offset, &mut frame_allocator);
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
