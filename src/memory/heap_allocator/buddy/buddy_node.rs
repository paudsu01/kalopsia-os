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
            next: KernelPointer::new(VirtualAddress::new(0x0)),
            previous: KernelPointer::new(VirtualAddress::new(0x0)),
        }
    }
}

/// bits 0-6: size, bit 7: available(1) or allocated(0)
/// Size: if a block is 2^k bytes big, size is stored as `k`
#[derive(Debug, Clone, Copy)]
pub struct BuddyHeader {
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

    pub fn change_availability(&mut self, new_value: bool) {
        *self = BuddyHeader::new(self.size(), new_value);
    }

    pub fn change_size(&mut self, new_size: u8) {
        *self = BuddyHeader::new(new_size, self.is_available());
    }
}
