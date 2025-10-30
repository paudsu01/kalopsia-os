#[test_case]
fn test_breakpoint_expn() {
    x86_64::instructions::interrupts::int3();
}
