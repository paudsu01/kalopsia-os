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

    pub fn set(&mut self, index: u64, entry: PageTableEntry) -> Result<(), &'static str> {
        if index >= MAX_ENTRIES {
            Err("Unable to set PTE")
        } else {
            self.entries[index as usize] = entry;
            Ok(())
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

impl PageTableLevel {
    pub fn from_u8(u: u8) -> Option<PageTableLevel> {
        match u {
            1 => Some(PageTableLevel::Level1),
            2 => Some(PageTableLevel::Level2),
            3 => Some(PageTableLevel::Level3),
            4 => Some(PageTableLevel::Level4),
            _ => None,
        }
    }
}

mod page_table_entry;
pub use page_table_entry::PTFlags;
pub use page_table_entry::PageTableEntry;
