use crate::memory::VirtualAddress;

/// Each free list's node contains the following information: `header`, `next` and `previous`.
/// Doubly linked list approach used
#[repr(C)]
#[allow(dead_code)]
pub struct BuddyNode {
    header: BuddyHeader,
    next: u64,     // null ptr(0) is no other node
    previous: u64, // null ptr(0) is no other node
}

#[allow(dead_code)]
impl BuddyNode {
    pub fn new(size: u8, available: bool) -> Self {
        BuddyNode {
            header: BuddyHeader::new(size, available),
            next: 0,
            previous: 0,
        }
    }

    pub fn next(&mut self) -> Option<&'static mut Self> {
        if self.next == 0 {
            None
        } else {
            unsafe { Some(BuddyNode::from_vaddr_ptr(VirtualAddress::new(self.next))) }
        }
    }

    pub fn previous(&mut self) -> Option<&'static mut Self> {
        if self.previous == 0 {
            None
        } else {
            unsafe {
                Some(BuddyNode::from_vaddr_ptr(VirtualAddress::new(
                    self.previous,
                )))
            }
        }
    }

    pub unsafe fn from_vaddr_ptr(addr: VirtualAddress) -> &'static mut Self {
        unsafe { &mut *(addr.as_u64() as *mut BuddyNode) }
    }
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
