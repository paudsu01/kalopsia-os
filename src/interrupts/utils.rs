use core::arch::asm;

/// Enable interrupts(uses the `sti` instruction)
pub fn enable() {
    unsafe {
        asm!("sti");
    }
}

/// Disable interrupts(uses the `cli` instruction)
pub fn disable() {
    unsafe {
        asm!("cli");
    }
}

pub fn without_interrupts<F: Fn()>(closure: F) {
    disable();
    closure();
    enable();
}
