use crate::{println, serial_println};

// Custom test framework setup: https://doc.rust-lang.org/beta/unstable-book/language-features/custom-test-frameworks.html
// Reference: https://os.phil-opp.com/testing/
pub fn custom_test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests:", tests.len());
    for test in tests {
        test();
        println!("..ok");
    }
    serial_println!("Completed all tests");
    exit_qemu(QEMUExitCode::Success);
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
    let exit_code: u32 = exit_code as u32;
    unsafe {
        asm!(
            "out dx, eax",
            in("dx") 0xf4,  // port number in DX
            in("eax") exit_code, // 4 byte exit code in EAX
        );
    }
}

mod serial;
pub use serial::_print;
