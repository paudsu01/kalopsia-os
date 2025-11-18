use core::ops::Add;

use crate::memory::page_table::PageSize;

/// VirtualAddress to represent an address
#[derive(Debug)]
#[allow(dead_code)]
pub struct VirtualAddress {
    pub addr: *const u8,
}

/// x86-64 virtual address breakdown for 4-level paging:
///
///  64               48         39          30          21          12              0
///  |------------------| --------- | --------- | --------- | --------- | ------------ |
///  | Sign Extension   |  L4 index |  L3 index | L2 index  | L1 index  | Offset       |
#[allow(dead_code)]
impl VirtualAddress {
    pub fn new(addr: u64) -> Self {
        VirtualAddress {
            addr: (addr as *const u8),
        }
    }

    pub fn as_u64(&self) -> u64 {
        self.addr as u64
    }

    pub fn get_lvl_4_index(&self) -> u64 {
        ((self.addr as u64) >> 39) & 0o777
    }

    pub fn get_lvl_3_index(&self) -> u64 {
        ((self.addr as u64) >> 30) & 0o777
    }

    pub fn get_lvl_2_index(&self) -> u64 {
        ((self.addr as u64) >> 21) & 0o777
    }

    pub fn get_lvl_1_index(&self) -> u64 {
        ((self.addr as u64) >> 12) & 0o777
    }

    pub fn get_offset(&self, offset_type: PageSize) -> u64 {
        match offset_type {
            PageSize::FourKiB => (self.addr as u64) & 0o7777,
            PageSize::TwoMiB => (self.addr as u64) & 0o7777777,
            PageSize::OneGiB => (self.addr as u64) & 0o7777777777,
        }
    }
}

/* Implement `Add` trait
 * Useful to translating vir addr to phys addr with bootloader's complete physical memory mapping with
 * offset approach chosen
 */
impl Add<u64> for VirtualAddress {
    type Output = Self;

    fn add(self, other: u64) -> Self {
        VirtualAddress {
            addr: (self.as_u64() + other) as *const u8,
        }
    }
}
