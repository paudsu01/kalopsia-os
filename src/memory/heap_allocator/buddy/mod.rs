use super::HEAP_LOG_SIZE;
use crate::memory::VirtualAddress;
use crate::utils::MutexWrapper;
use core::{
    alloc::{GlobalAlloc, Layout},
    ptr::null_mut,
};

mod buddy_node;
pub use buddy_node::BuddyNode;

pub const MIN_BLOCK_SIZE_LOG: u64 = 5; // 2^5=32 bytes

pub struct BinaryBuddyAllocator {
    free_lists: [Option<&'static mut BuddyNode>; (HEAP_LOG_SIZE - MIN_BLOCK_SIZE_LOG + 1) as usize],
}

impl Default for BinaryBuddyAllocator {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code)]
impl BinaryBuddyAllocator {
    pub const fn new() -> Self {
        BinaryBuddyAllocator {
            free_lists: [const { None }; (HEAP_LOG_SIZE - MIN_BLOCK_SIZE_LOG + 1) as usize],
        }
    }

    pub fn init(&mut self, heap_start: u64) {
        self.free_lists[0] =
            Some(unsafe { BuddyNode::from_vaddr_ptr(VirtualAddress::new(heap_start)) });
    }

    pub fn get_free_list(&self, size_log: u8) -> &Option<&'static mut BuddyNode> {
        &self.free_lists[(HEAP_LOG_SIZE - (size_log as u64)) as usize]
    }
}

// Map the assigned virtual memory region to physical frames
#[global_allocator]
pub static ALLOCATOR: MutexWrapper<BinaryBuddyAllocator> =
    MutexWrapper::new(BinaryBuddyAllocator::new());

unsafe impl GlobalAlloc for MutexWrapper<BinaryBuddyAllocator> {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        // TODO
        null_mut()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // TODO
        panic!("deallocation not supported");
    }
}

/// Very trivial way to do it but its fine for now
#[allow(dead_code)]
fn round_log(value: u64) -> u8 {
    let mut power: u8 = 0;
    let mut cvalue = 1;
    while cvalue < value {
        cvalue *= 2;
        power += 1;
    }
    power
}
