#![no_std]
#![no_main]

use core::panic::PanicInfo;
use kalopsia_os::{memory::PhysicalAddress, println};

// Custom panic handler since std lib is disabled
#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
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

    let addresses = [
        0xb8000,
        0x201008,
        0x0100_0020_1a10,
        boot_info.physical_memory_offset,
    ];

    for address in addresses {
        let virt = VirtualAddress::new(address);
        let phys = MEMORY.lock().translate_address(virt);
        println!("{:?} -> {:?}", address as *const u8, phys);
    }

    let mut dummy_allocator = kalopsia_os::memory::DummyAllocator;

    let _ = MEMORY.lock().map_4kib_page(
        VirtualAddress::new(0x0),
        PhysicalAddress::new(0xb8000),
        PTFlags::Write | PTFlags::Present,
        &mut dummy_allocator,
    );

    let page_ptr: *mut u64 = VirtualAddress::new(0x0).as_u64() as *mut u64;
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

    kalopsia_os::hlt();
}
