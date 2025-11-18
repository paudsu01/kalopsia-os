use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::registers::control::Cr3;

// mod recursive_paging;
// pub use recursive_paging::get_page_table_vaddr;

#[cfg(test)]
mod addr_translation_tests;

mod addr;
mod page_table;
pub use addr::{PhysicalAddress, VirtualAddress};
use page_table::{PageSize, PageTable, PageTableLevel};

struct Memory {
    offset: Option<u64>,
}

lazy_static! {
    static ref MEMORY: Mutex<Memory> = Mutex::new(Memory { offset: None });
}

pub fn init(physical_memory_offset: u64) {
    MEMORY.lock().offset = Some(physical_memory_offset);
}

/// Note: The memory module assumes the bootloader has mapped the whole physical memory. See
/// `Cargo.toml` -> bootloader -> features=["map_physical_memory"]
///  The memory is mapped such that v.addr = p.addr + offset
///  This offset needs to be passed as the argument to Memory
///  Must only be called once because of mutable reference
pub fn active_lvl_4_pt() -> &'static mut PageTable {
    let pt_physical_frame = Cr3::read();
    let lvl_4_pt_physical_addr = pt_physical_frame.0.start_address().as_u64();

    /* V.addr = P.Addr + offset (setup by the bootloader when paging enabled) */
    let v_addr = VirtualAddress::new(lvl_4_pt_physical_addr) + MEMORY.lock().offset.unwrap();
    let ptr = v_addr.addr as *mut PageTable;
    unsafe { &mut *ptr }
}

/// Traverse the page table to convert the given v.addr to p.addr
/// If no entry present, returns None
pub fn translate_address(addr: VirtualAddress) -> Option<PhysicalAddress> {
    let physical_memory_offset = MEMORY.lock().offset.unwrap();

    // get the different indexes to be used for different level page tables
    let indexes = [
        addr.get_lvl_4_index(),
        addr.get_lvl_3_index(),
        addr.get_lvl_2_index(),
        addr.get_lvl_1_index(),
    ];
    let pt_physical_frame = Cr3::read();
    let vaddr = pt_physical_frame.0.start_address().as_u64() + physical_memory_offset;
    let current_page_table = unsafe { &*(vaddr as *const PageTable) };
    // recursively find out the pfn entry to get the physical address
    get_physical_address(
        4,
        current_page_table,
        addr,
        &indexes,
        physical_memory_offset,
    )
}

/// Note that walk can stop early and offset can changes if huge page (2MiB and 1GiB) if PS is set to 1
/// For 1 GiB page -> PS can be set to 1 in P3,
/// For 2MiB page -> PS can be set to 1 in P2
fn get_physical_address(
    current_level: usize,
    current_page_table: &PageTable,
    addr: VirtualAddress,
    indexes: &[u64; 4],
    physical_memory_offset: u64,
) -> Option<PhysicalAddress> {
    let ptable_level: PageTableLevel =
        page_table::PageTableLevel::from_u8(current_level as u8).unwrap();
    let pte = current_page_table.get(indexes[4 - current_level]).unwrap();
    let pfn_physical_address: PhysicalAddress = pte.as_physical_address(ptable_level);

    match current_level {
        // level 1 PT gives the PFN so p.addr = PFN << 12 | offset
        1 => Some(PhysicalAddress::new(
            pfn_physical_address.as_u64() | addr.get_offset(PageSize::FourKiB),
        )),
        _ => {
            // no translation available if PTE is 0
            if pte.is_unused() {
                None
            // 2 MiB page table entry
            } else if current_level == 2 && pte.is_huge() {
                Some(PhysicalAddress::new(
                    pfn_physical_address.as_u64() | addr.get_offset(PageSize::TwoMiB),
                ))
            // 1 GiB page table entry
            } else if current_level == 3 && pte.is_huge() {
                // p.addr = PFN(4KiB based) << 12 | lvl 1 index | offset
                Some(PhysicalAddress::new(
                    pfn_physical_address.as_u64() | addr.get_offset(PageSize::OneGiB),
                ))
            } else {
                // v.addr for the page table of the next level
                let vaddr = pfn_physical_address.as_u64() + physical_memory_offset;
                get_physical_address(
                    current_level - 1,
                    unsafe { &*(vaddr as *const PageTable) },
                    addr,
                    indexes,
                    physical_memory_offset,
                )
            }
        }
    }
}
