use super::{HEAP_LOG_SIZE, HEAP_START};
use crate::memory::heap_allocator::buddy::buddy_node::BuddyHeader;
use crate::memory::VirtualAddress;
use crate::utils::KernelPointer;
use crate::utils::MutexWrapper;
use core::{
    alloc::{GlobalAlloc, Layout},
    ptr::null_mut,
};

mod buddy_node;
pub use buddy_node::BuddyNode;

pub const MIN_BLOCK_SIZE_LOG: u64 = 5; // 2^5=32 bytes

#[allow(dead_code)]
pub struct BinaryBuddyAllocator {
    free_lists:
        [Option<KernelPointer<BuddyNode>>; (HEAP_LOG_SIZE - MIN_BLOCK_SIZE_LOG + 1) as usize],
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

    pub fn init(&mut self, heap_start: KernelPointer<BuddyNode>) {
        unsafe { *(heap_start.ptr) = BuddyNode::new(HEAP_LOG_SIZE as u8, true) };
        self.free_lists[0] = Some(heap_start);
    }

    /// Index 0 of free list -> 1MiB blocks
    /// Index 1 -> 512Kib blocks
    /// and so on!
    fn get_free_list_index(&self, size_log: u8) -> Option<u8> {
        let size_log = size_log as u64;
        if !(MIN_BLOCK_SIZE_LOG..=HEAP_LOG_SIZE).contains(&size_log) {
            None
        } else {
            Some((HEAP_LOG_SIZE - size_log) as u8)
        }
    }

    /// Recursive function
    fn request_block(&mut self, block_size_log: u8, index: u8) -> Option<KernelPointer<BuddyNode>> {
        let free_list = self.free_lists[index as usize];
        match free_list {
            None => {
                // base case
                if block_size_log as u64 == HEAP_LOG_SIZE {
                    None
                // request a bigger block
                // recursive case
                } else {
                    let bigger_block: KernelPointer<BuddyNode> =
                        self.request_block(block_size_log + 1, index - 1)?;
                    // split it into two (return `block_one`, and add `block_two` to the free list)
                    let block_one = bigger_block;
                    let block_two = BinaryBuddyAllocator::buddy_address(block_one, block_size_log);
                    unsafe {
                        *(block_one.ptr) = BuddyNode::new(block_size_log, false);
                        *(block_two.ptr) = BuddyNode::new(block_size_log, true);
                    }
                    // add one back to the free list
                    self.free_lists[index as usize] = Some(block_two);
                    // return the other one to the user
                    Some(block_one)
                }
            }
            Some(kernel_pointer) => {
                let node: *mut BuddyNode = kernel_pointer.ptr;
                let next_node = unsafe { (*node).next };
                // If no next node, this is the only block in this free list, so we return this one
                if next_node.ptr.is_null() {
                    self.free_lists[index as usize] = None;
                } else {
                    let next_node_ptr = next_node.ptr;
                    unsafe {
                        (*next_node_ptr).previous = KernelPointer::new(VirtualAddress::null());
                    };
                    self.free_lists[index as usize] = Some(next_node);
                }
                Some(kernel_pointer)
            }
        }
    }

    /// Buddy address = buddy XOR 2^k, where 2^k is the size of the block
    /// Heap memory is thought of conceptually of size 1Mib starting from address 0.
    pub fn buddy_address(
        buddy_one: KernelPointer<BuddyNode>,
        size_log: u8,
    ) -> KernelPointer<BuddyNode> {
        let offset = HEAP_START;
        let v_vaddr = ((buddy_one.ptr as u64) - offset) ^ (1 << size_log);
        let a_vaddr = v_vaddr + offset;
        KernelPointer::new(VirtualAddress::new(a_vaddr))
    }
}

// Map the assigned virtual memory region to physical frames
#[global_allocator]
pub static ALLOCATOR: MutexWrapper<BinaryBuddyAllocator> =
    MutexWrapper::new(BinaryBuddyAllocator::new());

unsafe impl GlobalAlloc for MutexWrapper<BinaryBuddyAllocator> {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        let mut buddy_allocator = self.lock();

        let size_of_header = core::mem::size_of::<BuddyHeader>();
        // Allocated block size needs space for header information as well as space for node
        // information(next, previous) potentially once it is freed
        // So, MIN_BLOCK_SIZE_LOG is 5 (32 bytes = 2^5)
        let mut block_size_log = round_log((size_of_header + _layout.size()) as u64);
        if block_size_log < (MIN_BLOCK_SIZE_LOG as u8) {
            block_size_log = MIN_BLOCK_SIZE_LOG as u8;
        }

        let index = buddy_allocator.get_free_list_index(block_size_log);
        let Some(index) = index else {
            return null_mut();
        };
        let block = buddy_allocator.request_block(block_size_log, index);
        match block {
            None => null_mut(),
            Some(kernel_ptr) => unsafe {
                *(kernel_ptr.ptr) = BuddyNode::new(block_size_log, false);
                // move pointer to account for header info stored
                (kernel_ptr.ptr as *mut u8).add(size_of_header)
            },
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
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
