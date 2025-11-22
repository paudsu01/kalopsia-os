#![no_std]
#![no_main]

use core::panic::PanicInfo;
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
    use kalopsia_os::memory::{PTFlags, VirtualAddress, MEMORY};

    println!("Hello World!, ");
    println!("this is {}", "kalopsia-os");

    let mut frame_allocator = unsafe {
        kalopsia_os::memory::BootInfoFrameAllocator::init(usable_frames(&boot_info.memory_map))
    };

    let addresses = [
        0xb8000,
        0x201008,
        0x0100_0020_1a10,
        boot_info.physical_memory_offset,
        0xdeadbeaf000,
    ];

    let x = MEMORY
        .lock()
        .map_4kib_page(
            VirtualAddress::new(0xdeadbeaf000),
            PhysicalAddress::new(0xb8000),
            PTFlags::Write | PTFlags::Present,
            &mut frame_allocator,
        )
        .unwrap();
    x.flush();

    for address in addresses {
        let virt = VirtualAddress::new(address);
        let phys = MEMORY.lock().translate_address(virt);
        println!("{:?} -> {:?}", address as *const u8, phys);
    }

    let page_ptr: *mut u64 = VirtualAddress::new(0xdeadbeaf000).as_u64() as *mut u64;
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

    kalopsia_os::hlt();
}
