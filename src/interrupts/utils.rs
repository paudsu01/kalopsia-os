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

/// Read the IF flag from RLAGS
fn if_flag() -> u8 {
    let rflags: u64;
    unsafe {
        asm!(
            "pushfq; pop {}",
            out(reg) rflags,
            options(nomem, preserves_flags),
        );
    }
    // IF is bit 9 in RFLAGS
    (rflags & (1 << 9)) as u8
}

pub fn without_interrupts<F: Fn()>(closure: F) {
    let if_flag = if_flag();
    if if_flag == 1 {
        disable();
    }
    closure();

    if if_flag == 1 {
        enable();
    }
}
