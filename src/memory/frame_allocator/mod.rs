use x86_64::structures::paging::{FrameAllocator, PhysFrame, Size4KiB};

#[allow(dead_code)]
pub struct DummyAllocator;

#[allow(dead_code)]
unsafe impl FrameAllocator<Size4KiB> for DummyAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        None
    }
}
