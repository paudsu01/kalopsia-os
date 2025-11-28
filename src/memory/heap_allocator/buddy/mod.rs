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
        let free_list: Option<KernelPointer<BuddyNode>> = self.free_lists[index as usize];
        if free_list.is_none() {
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
        } else {
            // pop the first block from the front of the free list
            unsafe { self.pop(index) }
        }
    }

    fn free_block(&mut self, buddy_one: KernelPointer<BuddyNode>) {
        let buddy_one_ptr = buddy_one.ptr;
        let buddy_size_log = unsafe { (*buddy_one_ptr).header.size() };

        // base case: cannot merge max sized block
        if (buddy_size_log as u64) == HEAP_LOG_SIZE {
            unsafe {
                self.append(buddy_one);
            }
            return;
        }

        // get the buddy otherwise
        let buddy_two = BinaryBuddyAllocator::buddy_address(buddy_one, buddy_size_log);
        let buddy_two_ptr = buddy_two.ptr;

        let buddy_two_available = unsafe { (*buddy_two_ptr).header.is_available() };
        if buddy_two_available {
            // if it is available, it must be in a free list
            // remove from the free list
            unsafe {
                self.remove(buddy_two)
                    .expect("buddy was marked available but not found in free list");
            }
            // merge the two buddies into one big block
            let big_block: KernelPointer<BuddyNode> =
                unsafe { self.merge(buddy_one, buddy_two, buddy_size_log) };
            // recursively try to free the bigger block
            self.free_block(big_block);
        } else {
            // if buddy is not available(free), just append the block to its relevant free list
            unsafe {
                self.append(buddy_one);
            }
        }
    }

    /// Buddy address = buddy XOR 2^k, where 2^k is the size of the block
    /// Heap memory is thought of conceptually of size 1Mib starting from address 0.
    fn buddy_address(
        buddy_one: KernelPointer<BuddyNode>,
        size_log: u8,
    ) -> KernelPointer<BuddyNode> {
        let offset = HEAP_START;
        let v_vaddr = ((buddy_one.ptr as u64) - offset) ^ (1_u64 << size_log);
        let a_vaddr = v_vaddr + offset;
        KernelPointer::new(VirtualAddress::new(a_vaddr))
    }

    /// Pop's from the front of the free list
    /// Free list is selected based on the `free_list_index`
    /// Doesn't change any `BuddyHeader` values
    unsafe fn pop(&mut self, free_list_index: u8) -> Option<KernelPointer<BuddyNode>> {
        let index = free_list_index as usize;
        if index >= self.free_lists.len() {
            None
        } else {
            let node_kernel_pointer: KernelPointer<BuddyNode> = self.free_lists[index]?;
            let node: *mut BuddyNode = node_kernel_pointer.ptr;
            let next_node = unsafe { (*node).next };
            // If no next node, this is the only block in this free list, so we return this one
            if next_node.ptr.is_null() {
                self.free_lists[index] = None;
            } else {
                let next_node_ptr = next_node.ptr;
                unsafe {
                    (*next_node_ptr).previous = KernelPointer::new(VirtualAddress::null());
                };
                self.free_lists[index] = Some(next_node);
            }
            Some(node_kernel_pointer)
        }
    }

    /// Append a block to the front of its free list
    /// Free list is selected based on the header info stored in the block itself!
    /// Marks block as available
    unsafe fn append(&mut self, node: KernelPointer<BuddyNode>) {
        let node_ptr: *mut BuddyNode = node.ptr;
        let size_log = unsafe { (*node_ptr).header.size() };
        // Set block to available
        unsafe { *node_ptr = BuddyNode::new(size_log, true) };

        let index = self
            .get_free_list_index(size_log)
            // unwrap shouldn't fail
            .expect("Free list index based on the size_log failed") as usize;

        // Get the correct doubly linked free list based on the `index`
        let head_node_ptr: Option<KernelPointer<BuddyNode>> = self.free_lists[index];

        if let Some(head_node_ptr) = head_node_ptr {
            let head_ptr = head_node_ptr.ptr;
            unsafe {
                (*head_ptr).previous = node;
                (*node_ptr).next = head_node_ptr;
            }
        }
        // Add to front of the free list
        self.free_lists[index] = Some(node);
    }

    /// Removes the selected block from its free list
    /// Unsafe because the user must guarantee that the block is already in its relevant free list
    unsafe fn remove(
        &mut self,
        node_kernel_ptr: KernelPointer<BuddyNode>,
    ) -> Option<KernelPointer<BuddyNode>> {
        let ptr = node_kernel_ptr.ptr;
        let previous_node = unsafe { (*ptr).previous };
        let next_node = unsafe { (*ptr).next };
        let previous_node_ptr = previous_node.ptr;
        let next_node_ptr = next_node.ptr;

        let index = {
            let size_log = unsafe { (*ptr).header.size() };
            self.get_free_list_index(size_log).unwrap()
        };

        // is at the beginning of the free list
        if previous_node_ptr.is_null() {
            unsafe { self.pop(index) }
        } else {
            unsafe { (*previous_node_ptr).next = next_node };
            if !(next_node_ptr.is_null()) {
                unsafe { (*next_node_ptr).previous = previous_node };
            }
            Some(node_kernel_ptr)
        }
    }

    unsafe fn merge(
        &mut self,
        buddy_one: KernelPointer<BuddyNode>,
        buddy_two: KernelPointer<BuddyNode>,
        size_log: u8,
    ) -> KernelPointer<BuddyNode> {
        unsafe {
            // see which buddy is smaller
            if buddy_one.ptr < buddy_two.ptr {
                // delete(zero-out) the node info from the other buddy
                *(buddy_two.ptr) = BuddyNode::new(0, false);
                *(buddy_one.ptr) = BuddyNode::new(size_log + 1, true);
                buddy_one
            } else {
                *(buddy_one.ptr) = BuddyNode::new(0, false);
                *(buddy_two.ptr) = BuddyNode::new(size_log + 1, true);
                buddy_two
            }
        }
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

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        let mut buddy_allocator = self.lock();

        // get the start ptr of the block where the header info is
        let size_of_header = core::mem::size_of::<BuddyHeader>();
        let buddy_node_ptr = unsafe { _ptr.offset(-(size_of_header as isize)) } as *mut BuddyNode;

        let buddy_node_kernel_ptr = KernelPointer::new(VirtualAddress::new(buddy_node_ptr as u64));
        // recursively free the block coalescing into bigger blocks if possible
        buddy_allocator.free_block(buddy_node_kernel_ptr);
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
