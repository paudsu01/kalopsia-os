use core::slice::Iter;

const MAX_ENTRIES: u64 = 512;
#[repr(transparent)]
pub struct PageTable {
    entries: [PageTableEntry; MAX_ENTRIES as usize],
}

impl PageTable {
    pub fn iter(&self) -> Iter<'_, PageTableEntry> {
        self.entries.iter()
    }

    pub fn get(&self, index: u64) -> Option<PageTableEntry> {
        if index >= MAX_ENTRIES {
            None
        } else {
            Some(self.entries[index as usize])
        }
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum PageSize {
    FourKiB,
    TwoMiB,
    OneGiB,
}

#[derive(Copy, Clone)]
pub enum PageTableLevel {
    Level1,
    Level2,
    Level3,
    Level4,
}

// Source: Content from https://blog.zolutal.io/understanding-paging/
// Here is what a Page Table Entry looks like in any level page table in x86-64:
//                                                                 Present ──────┐
//                                                             Read/Write ──────┐|
//                                                       User/Supervisor ──────┐||
//                                                   Page Write Through ──────┐|||
//                                                Page Cache Disabled ──────┐ ||||
//                                                          Accessed ──────┐| ||||
//                                                            Dirty ──────┐|| ||||
//                                                       Huge Page ──────┐||| ||||
//                                                          Global ────┐ |||| ||||
// ┌─ NX                                                  Ignored ──┬┬┐| |||| ||||
// |┌───────────┐ ┌───────────────────────────────────────────────┐ |||| |||| ||||
// ||  Ignored  | |                Physical Address               | |||| |||| ||||
// ||           | |                                               | |||| |||| ||||
// 0000 0000 0000 0000 0000 0000 0000 0001 0010 0011 1111 1100 1010 0000 0110 0111
//        56        48        40        32        24        16         8         0
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct PageTableEntry {
    value: u64,
}

#[allow(dead_code)]
impl PageTableEntry {
    // Mask for 4KiB pages (Clears bits 0-11, 52+)
    const PFN_4KIB_MASK: u64 = 0x000F_FFFF_FFFF_F000;

    // Mask for 2MiB pages (Clears bits 0-20, 52+)
    const PFN_2MIB_MASK: u64 = 0x000F_FFFF_FFE0_0000;

    // Mask for 1GiB pages (Clears bits 0-29, 52+)
    const PFN_1GIB_MASK: u64 = 0x000F_FFFF_C000_0000;

    /// Argument `pfn`: The pfn is always and must be page aligned so it would be `physical_address >> 12`
    /// `pfn` means 4KiB based PFN
    pub fn new(pfn: u64, flags: u64) -> PageTableEntry {
        // 52-bit physical address limit means PFN fits in 40 bits (52 - 12).
        // 0x000F_FFFF_FFFF_F is the mask for the lower 52 bits shifted down by 12.
        let pfn_mask = 0x0000_00FF_FFFF_FFFF;

        PageTableEntry {
            value: (pfn & pfn_mask) << 12 | flags,
        }
    }

    pub fn as_physical_address(&self, page_table_level: PageTableLevel) -> *const u8 {
        (match page_table_level {
            PageTableLevel::Level1 => self.value & PageTableEntry::PFN_4KIB_MASK,
            PageTableLevel::Level2 => {
                let mask = if self.is_huge() {
                    PageTableEntry::PFN_2MIB_MASK
                } else {
                    PageTableEntry::PFN_4KIB_MASK
                };
                self.value & mask
            }
            PageTableLevel::Level3 => {
                let mask = if self.is_huge() {
                    PageTableEntry::PFN_1GIB_MASK
                } else {
                    PageTableEntry::PFN_4KIB_MASK
                };
                self.value & mask
            }
            PageTableLevel::Level4 => self.value & PageTableEntry::PFN_4KIB_MASK,
        }) as *const u8
    }

    pub fn is_present(&self) -> bool {
        (self.value & 1) != 0
    }

    pub fn is_writable(&self) -> bool {
        ((self.value >> 1) & 1) != 0
    }

    pub fn is_user_accessible(&self) -> bool {
        ((self.value >> 2) & 1) != 0
    }

    pub fn is_write_through(&self) -> bool {
        ((self.value >> 3) & 1) != 0
    }

    pub fn is_cache_disabled(&self) -> bool {
        ((self.value >> 4) & 1) != 0
    }

    pub fn is_accessed(&self) -> bool {
        ((self.value >> 5) & 1) != 0
    }

    pub fn is_dirty(&self) -> bool {
        ((self.value >> 6) & 1) != 0
    }

    pub fn is_huge(&self) -> bool {
        ((self.value >> 7) & 1) != 0
    }

    pub fn is_global(&self) -> bool {
        ((self.value >> 8) & 1) != 0
    }

    pub fn is_nx(&self) -> bool {
        ((self.value >> 63) & 1) != 0
    }

    pub fn is_unused(&self) -> bool {
        self.value == 0
    }
}

/* Flags for each page table entry */
#[repr(u64)]
#[allow(dead_code)]
pub enum PTFlags {
    Present = 1,
    Write = 1 << 1,
    UserAccess = 1 << 2,
    WriteThrough = 1 << 3,
    DisableCache = 1 << 4,
    HugePage = 1 << 7,
    Global = 1 << 8,
    NoExecute = 1_u64 << 63,
}
