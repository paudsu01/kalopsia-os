#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kalopsia_os::test_framework::custom_test_runner)]
#![reexport_test_harness_main = "test_main"]

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    kalopsia_os::panic_handler(info);
}

#[test_case]
fn test_println() {
    kalopsia_os::println!("integration testing: test_println");
}
