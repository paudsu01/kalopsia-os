/// PhysicalAddress
#[derive(Debug)]
#[allow(dead_code)]
pub struct PhysicalAddress {
    pub addr: *const u8,
}

#[allow(dead_code)]
impl PhysicalAddress {
    pub fn new(addr: u64) -> Self {
        PhysicalAddress {
            addr: (addr as *const u8),
        }
    }

    pub fn as_u64(&self) -> u64 {
        self.addr as u64
    }
}
