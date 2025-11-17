use core::ops::Add;

/// VirtualAddress to represent an address
#[allow(dead_code)]
pub struct VirtualAddress {
    pub addr: *const u8,
}

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
