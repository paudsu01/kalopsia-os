use crate::memory::VirtualAddress;
/// Each free list's node contains the following information: `header`, `next` and `previous`.
/// Doubly linked list approach used
#[repr(C)]
#[allow(dead_code)]
struct BuddyNode {
    header: BuddyHeader,
    next: *const u8,     // null ptr is no other node
    previous: *const u8, // null ptr is no other node
}

/// bits 0-6: size, bit 7: available(1) or allocated(0)
/// Size: if a block is 2^k bytes big, size is stored as `k`
#[allow(dead_code)]
struct BuddyHeader {
    header: u8,
}

#[allow(dead_code)]
impl BuddyHeader {
    pub fn new(size: u8, available: bool) -> Self {
        BuddyHeader {
            header: ((available as u8) << 7 | size),
        }
    }

    pub fn is_available(&self) -> bool {
        (self.header >> 7) == 1
    }

    pub fn size(&self) -> u8 {
        self.header & 0b01111111
    }
}

#[allow(dead_code)]
impl BuddyNode {
    pub unsafe fn from_vaddr_ptr(addr: VirtualAddress) -> &'static mut Self {
        unsafe { &mut *(addr.as_u64() as *mut BuddyNode) }
    }
}
