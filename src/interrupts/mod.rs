use crate::{print, println};
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

/// 8259 PIC
mod pic;
pub use pic::{init_pics, Interrupts, PICS};

mod utils;
pub use utils::enable;
pub use utils::without_interrupts;

#[cfg(test)]
mod interrupts_tests;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        init_handlers(&mut idt);
        idt
    };
}

pub fn init_idt() {
    // uses the `lidt` instruction to load our IDT
    IDT.load();
}

pub fn init_handlers(idt: &mut InterruptDescriptorTable) {
    idt.breakpoint.set_handler_fn(breakpoint_exception_handler);
    idt.divide_error
        .set_handler_fn(divide_by_zero_exception_handler);
    unsafe {
        idt.double_fault
            .set_handler_fn(double_fault_handler)
            // need to switch stack on double fault to prevent kernel stack overflow -> triple fault
            .set_stack_index(crate::gdt::DOUBLE_FAULT_STACK_INDEX as u16);
    }
    idt.page_fault.set_handler_fn(page_fault_handler);
    idt[pic::Interrupts::Timer as u8].set_handler_fn(timer_interrupt_handler);
}

/* For all the exception handlers, the x86-interrupt calling convention hides most details of
 * exception handling
 *  -> such as calling the `iretq` instruction afterwards which restores the state of the
 *    interrupted process
 */

extern "x86-interrupt" fn breakpoint_exception_handler(frame: InterruptStackFrame) {
    println!("Breakpoint exception: {:#?}", frame);
}

extern "x86-interrupt" fn divide_by_zero_exception_handler(frame: InterruptStackFrame) {
    println!("You tried to divide by zero: {:#?}", frame);
    panic!("Divide by zero error: {:#?}", frame);
}

extern "x86-interrupt" fn double_fault_handler(frame: InterruptStackFrame, error_code: u64) -> ! {
    panic!(
        "double fault exn: error code: {}\nframe:\n{:#?}",
        error_code, frame
    );
}

extern "x86-interrupt" fn page_fault_handler(
    frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    panic!("Page fault, error_code: {:#?}, {:#?}", error_code, frame);
}

extern "x86-interrupt" fn timer_interrupt_handler(_frame: InterruptStackFrame) {
    print!(".");
    PICS.lock().end_of_interrupt(Interrupts::Timer as u8);
}
