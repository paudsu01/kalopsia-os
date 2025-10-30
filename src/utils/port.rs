use core::arch::asm;

/// interface to read/write to ports in x86_64
pub struct Port {
    addr: u16,
}

impl Port {
    pub fn new(addr: u16) -> Self {
        Port { addr }
    }

    /// Reads a byte from the port address
    pub fn readb(&mut self) -> u8 {
        let value: u8;
        unsafe {
            asm!(
                "in al, dx",
                in("dx") self.addr,
                out("al") value,
            );
        }
        value
    }

    /// Writes a byte from the port address
    pub fn writeb(&mut self, val: u8) {
        unsafe {
            asm!(
                "out dx, al",
                in("dx") self.addr,
                in("al") val,
            );
        }
    }

    /// Reads a word(2 bytes) from the port address
    pub fn readw(&mut self) -> u16 {
        let value: u16;
        unsafe {
            asm!(
            "in ax, dx",
            in("dx") self.addr,
            out("ax") value,
            );
        }
        value
    }

    /// Writes a word(2 bytes) from the port address
    pub fn writew(&mut self, val: u16) {
        unsafe {
            asm!(
            "out dx, ax",
            in("dx") self.addr,
            in("ax") val,
            );
        }
    }

    /// Reads a long(4 bytes) from the port address
    pub fn readl(&mut self) -> u32 {
        let value: u32;
        unsafe {
            asm!(
                "in eax, dx",
                in("dx") self.addr,
                out("eax") value,
            );
        }
        value
    }

    /// Writes a long(4 bytes) from the port address
    pub fn writel(&mut self, val: u32) {
        unsafe {
            asm!(
                "out dx, eax",
                in("dx") self.addr,
                in("eax") val,
            );
        }
    }
}
