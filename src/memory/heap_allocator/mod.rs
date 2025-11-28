use crate::memory::{FrameAllocator, PTFlags, PhysicalAddress, VirtualAddress, MEMORY};
use crate::utils::KernelPointer;
use x86_64::structures::paging::Size4KiB;

/// Start of heap's virtual memory region
pub const HEAP_START: u64 = 0x_4444_4444_0000;
/// Heap size: 1MiB: 2^20 bytes
pub const HEAP_LOG_SIZE: u64 = 20;
pub const HEAP_SIZE: u64 = u64::pow(2, HEAP_LOG_SIZE as u32);

mod buddy;
use buddy::ALLOCATOR;

/// Map the assigned virtual memory region to physical frames
#[allow(dead_code)]
pub fn init_heap(frame_allocator: &mut impl FrameAllocator<Size4KiB>) -> Result<(), &'static str> {
    let four_kib = u64::pow(2, 12); // 2^12 bytes = 4KiB
    let total_pages = HEAP_SIZE >> 12;

    let mut current_vaddr = HEAP_START;
    let flags = PTFlags::Write | PTFlags::Present;

    for _ in 0..total_pages {
        // allocate a physical frame for the current virtual page
        let frame = frame_allocator.allocate_frame();
        let Some(frame) = frame else {
            return Err("heap init: allocation failed");
        };
        // map the virtual page to the recently allocated frame
        unsafe {
            MEMORY
                .lock()
                .map_4kib_page(
                    VirtualAddress::new(current_vaddr),
                    PhysicalAddress::new(frame.start_address().as_u64()),
                    flags,
                    frame_allocator,
                )?
                .flush();
        }

        // vaddr for the next virtual memory page
        current_vaddr += four_kib;
    }

    ALLOCATOR
        .lock()
        .init(KernelPointer::new(VirtualAddress::new(HEAP_START)));
    Ok(())
}
