use crate::println;
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

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
    loop {}
}

#[cfg(test)]
mod interrupts_tests;
