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
#[allow(dead_code)]
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
    ((rflags & 0x0200) >> 9) as u8
}

pub fn without_interrupts<F: Fn()>(closure: F) {
    let rflags: u64;
    // save rflags (including IF)
    unsafe {
        asm!(
            "pushfq",
            "pop {rflags}",
            "cli",
            rflags = out(reg) rflags,
            options(nomem, preserves_flags)
        );
    }

    closure();

    // restore rflags
    unsafe {
        asm!(
            "push {rflags}",
            "popfq",
            rflags = in(reg) rflags,
            options(nomem, preserves_flags)
        );
    }
}
