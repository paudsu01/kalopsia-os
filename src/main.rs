#![no_std]
#![no_main]

extern crate alloc;

use alloc::boxed::Box;
use core::panic::PanicInfo;
use x86_64::structures::paging::{FrameAllocator, Size4KiB};

use kalopsia_os::{
    interrupts,
    memory::{usable_frames, PhysicalAddress},
    println,
};

// Custom panic handler since std lib is disabled
#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    interrupts::disable();
    println!("{_info}");
    kalopsia_os::hlt();
}

/** `_start` function
 * Custom entry point to overwite default rust's crt0 entry point
 * Disabled name mangling to help the linker.
 * use extern "C" so that the compiler uses C calling convention for this function
 */
#[unsafe(no_mangle)]
pub extern "C" fn _start(boot_info: &'static bootloader::BootInfo) -> ! {
    kalopsia_os::init(boot_info.physical_memory_offset);
    let mut frame_allocator = unsafe {
        kalopsia_os::memory::BootInfoFrameAllocator::init(usable_frames(&boot_info.memory_map))
    };
    kalopsia_os::memory::init_heap(&mut frame_allocator).expect("Init: Heap init failed");

    println!("Hello World!, ");
    println!("this is {}", "kalopsia-os");

    example_mapping(&mut frame_allocator);
    kalopsia_os::hlt();
}

fn example_mapping(frame_allocator: &mut impl FrameAllocator<Size4KiB>) {
    use kalopsia_os::memory::{PTFlags, VirtualAddress, MEMORY};
    //let address = boot_info.physical_memory_offset;
    let address = 0x0;
    unsafe {
        MEMORY
            .lock()
            .map_4kib_page(
                VirtualAddress::new(address),
                PhysicalAddress::new(0xb8000),
                PTFlags::Write | PTFlags::Present,
                frame_allocator,
            )
            .unwrap()
            .flush();
    }

    let page_ptr: *mut u64 = address as *mut u64;
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };
}
