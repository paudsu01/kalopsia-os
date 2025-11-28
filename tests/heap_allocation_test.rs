#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kalopsia_os::test_framework::custom_test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::BootInfo;

extern crate alloc;

#[unsafe(no_mangle)]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    // Init memory: paging
    kalopsia_os::memory::init(boot_info.physical_memory_offset);
    let mut frame_allocator = unsafe {
        kalopsia_os::memory::BootInfoFrameAllocator::init(kalopsia_os::memory::usable_frames(
            &boot_info.memory_map,
        ))
    };
    // Init heap
    kalopsia_os::memory::init_heap(&mut frame_allocator).expect("Init: Heap init failed");
    // Run test
    test_main();
    kalopsia_os::hlt();
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    kalopsia_os::panic_handler(info);
}

#[test_case]
fn allocation_box() {
    use alloc::boxed::Box;

    let a = Box::new(42);
    let mut b = Box::new(-12);

    assert_eq!(*a, 42);
    assert_eq!(*b, -12);

    *b = 100;
    assert_eq!(*b, 100);
}

#[test_case]
fn allocation_vec() {
    use alloc::vec::Vec;

    let mut v = Vec::new();
    // Push elements and check
    v.push(1);
    v.push(2);
    v.push(3);

    assert_eq!(v.len(), 3);
    assert_eq!(v[0], 1);
    assert_eq!(v[1], 2);
    assert_eq!(v[2], 3);

    // Modify and check
    v[1] = 20;
    assert_eq!(v[1], 20);

    // Pop and check
    assert_eq!(v.pop(), Some(3));
    assert_eq!(v.len(), 2);
}

#[test_case]
fn allocation_string() {
    use alloc::string::String;

    let mut s = String::new();
    s.push_str("Hello");
    s.push(',');
    s.push(' ');
    s.push_str("world!");

    assert_eq!(s, "Hello, world!");

    // Modify contents
    s.push(' ');
    s.push_str("Rust");
    assert_eq!(s, "Hello, world! Rust");
}

#[test_case]
fn stress_large_vec() {
    use alloc::vec::Vec;

    let mut v = Vec::new();

    // Push many elements to force reallocations
    for i in 0..10_000 {
        v.push(i);
    }

    assert_eq!(v.len(), 10_000);

    // Verify a few random positions
    assert_eq!(v[0], 0);
    assert_eq!(v[1234], 1234);
    assert_eq!(v[9999], 9999);

    // Pop down to make sure shrinking works too
    for _ in 0..5_000 {
        let x = v.pop();
        assert!(x.is_some());
    }

    assert_eq!(v.len(), 5_000);
}

#[test_case]
fn many_boxes() {
    use alloc::boxed::Box;
    use kalopsia_os::memory::HEAP_SIZE;
    for i in 0..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
}
