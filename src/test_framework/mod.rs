use crate::{println, serial_print, serial_println, utils::Port};

// Custom test framework setup: https://doc.rust-lang.org/beta/unstable-book/language-features/custom-test-frameworks.html
// Reference: https://os.phil-opp.com/testing/
pub fn custom_test_runner(tests: &[&dyn Testable]) {
    serial_println!("\nRunning {} tests:\n", tests.len());
    for test in tests {
        test.run();
        println!("..ok");
    }
    serial_println!("\nCompleted all tests");
    exit_qemu(QEMUExitCode::Success);
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T: Fn()> Testable for T {
    fn run(&self) {
        serial_print!("Running {}", core::any::type_name::<T>());
        self();
        serial_println!(" ..[ok]");
    }
}

#[allow(unused)]
pub enum QEMUExitCode {
    Success = 0x10,
    Failure = 0x11,
}

#[allow(unused)]
pub fn exit_qemu(exit_code: QEMUExitCode) {
    // Check .config/cargo.toml: we provide `isa-debug-exit` argument during `cargo test` with
    // iobase `0xf4` of port size of 4 bytes
    // We basically produce the `out` assembly instruction which writes the status code to the port
    // 0xf4
    use core::arch::asm;
    let mut port = Port::new(0xf4);
    let exit_code: u32 = exit_code as u32;
    port.writel(exit_code);
}

mod serial;
pub use serial::_print;
