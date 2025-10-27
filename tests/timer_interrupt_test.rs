#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kalopsia_os::test_framework::custom_test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(kalopsia_os::gdt::DOUBLE_FAULT_STACK_INDEX as u16);
        }
        idt[kalopsia_os::interrupts::Interrupts::Timer as u8]
            .set_handler_fn(timer_interrupt_handler);
        idt
    };
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    kalopsia_os::serial_print!("\nRunning tests/timer_interrupt_test.rs");
    kalopsia_os::gdt::init();
    IDT.load();

    kalopsia_os::interrupts::init_pics();
    kalopsia_os::interrupts::enable();
    kalopsia_os::hlt();
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    kalopsia_os::panic_handler(info);
}

extern "x86-interrupt" fn double_fault_handler(_frame: InterruptStackFrame, _error_code: u64) -> ! {
    panic!();
}

extern "x86-interrupt" fn timer_interrupt_handler(_frame: InterruptStackFrame) {
    kalopsia_os::serial_println!(" ..[ok]\n");
    kalopsia_os::exit_qemu(kalopsia_os::QEMUExitCode::Success);
}
