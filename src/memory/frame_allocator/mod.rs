use bootloader::bootinfo::{MemoryMap, MemoryRegionType};
use x86_64::structures::paging::{FrameAllocator, PhysFrame, Size4KiB};
use x86_64::PhysAddr;

/// Based on phil opp's blog idea
// TODO: not quite convinced with my setup
// * need a way to deallocate
// * if a frame gets mapped in the page table, that should also not be usable and this allocator
// doesn't handle that
#[allow(dead_code)]
pub struct BootInfoFrameAllocator<T>
where
    T: Iterator<Item = PhysFrame<Size4KiB>>,
{
    usable_frames: T,
}

impl<T> BootInfoFrameAllocator<T>
where
    T: Iterator<Item = PhysFrame<Size4KiB>>,
{
    /// # Safety
    /// Usable frame is not guaranteed to be unused!
    /// Caller needs to guarantee that
    pub unsafe fn init(usable_frames: T) -> Self {
        BootInfoFrameAllocator { usable_frames }
    }
}

unsafe impl<T> FrameAllocator<Size4KiB> for BootInfoFrameAllocator<T>
where
    T: Iterator<Item = PhysFrame<Size4KiB>>,
{
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        self.usable_frames.next()
    }
}

/// From phil opp's blog: source: https://os.phil-opp.com/paging-implementation/#allocating-frames
pub fn usable_frames(memory_map: &'static MemoryMap) -> impl Iterator<Item = PhysFrame> {
    // get usable regions from memory map
    let regions = memory_map.iter();
    let usable_regions = regions.filter(|r| r.region_type == MemoryRegionType::Usable);
    // map each region to its address range
    let addr_ranges = usable_regions.map(|r| r.range.start_addr()..r.range.end_addr());
    // transform to an iterator of frame start addresses
    let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));
    // create `PhysFrame` types from the start addresses
    frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
}
