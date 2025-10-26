#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kalopsia_os::test_framework::custom_test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

use kalopsia_os::serial_print;
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
        idt
    };
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    kalopsia_os::println!("integration testing: stack_overflow");

    kalopsia_os::gdt::init();
    IDT.load();

    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    kalopsia_os::panic_handler(info);
}

// Need to check if stack overflow case is handled properly by switching the stack while handling the double fault handler
#[test_case]
fn test_stack_overflow() {
    stack_overflow();
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow();
    serial_print!("Should not reach here");
}

extern "x86-interrupt" fn double_fault_handler(_frame: InterruptStackFrame, _error_code: u64) -> ! {
    kalopsia_os::serial_println!(" ..[ok]");
    kalopsia_os::exit_qemu(kalopsia_os::QEMUExitCode::Success);
    loop {}
}
