use crate::{memory::VirtualAddress, utils::KernelPointer};

/// Each free list's node contains the following information: `header`, `next` and `previous`.
/// Doubly linked list approach used
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BuddyNode {
    pub header: BuddyHeader,
    pub next: KernelPointer<BuddyNode>,
    pub previous: KernelPointer<BuddyNode>,
}

#[allow(dead_code)]
impl BuddyNode {
    pub fn new(size: u8, available: bool) -> Self {
        BuddyNode {
            header: BuddyHeader::new(size, available),
            next: KernelPointer::new(VirtualAddress::null()),
            previous: KernelPointer::new(VirtualAddress::null()),
        }
    }
}

/// Size: if a block is 2^k bytes big, size is stored as `k`
/// u8 would have been enough to store both information, but padding will be added anyways(58 bits), so
/// might as well make them u32 each to change it easily
#[derive(Debug, Clone, Copy)]
pub struct BuddyHeader {
    available: u32,
    size: u32,
}

#[allow(dead_code)]
impl BuddyHeader {
    pub fn new(size: u8, available: bool) -> Self {
        BuddyHeader {
            available: available as u32,
            size: size as u32,
        }
    }

    pub fn is_available(&self) -> bool {
        self.available == 1
    }

    pub fn size(&self) -> u8 {
        self.size as u8
    }
}
