use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::registers::control::Cr3;

// mod recursive_paging;
// pub use recursive_paging::get_page_table_vaddr;

mod page_table;
mod vaddr;
use page_table::PageTable;
pub use vaddr::VirtualAddress;

struct Memory {
    offset: Option<u64>,
}

lazy_static! {
    static ref MEMORY: Mutex<Memory> = Mutex::new(Memory { offset: None });
}

pub fn init(physical_memory_offset: u64) {
    MEMORY.lock().offset = Some(physical_memory_offset);
}

/* Note: The memory module assumes the bootloader has mapped the whole physical memory. See
 * `Cargo.toml` -> bootloader -> features=["map_physical_memory"]
 * The memory is mapped such that v.addr = p.addr + offset
 * This offset needs to be passed as the argument to Memory
 * Must only be called once because of mutable reference
 */
// This approach  the `map_physical_memory`
pub fn active_lvl_4_pt() -> &'static mut PageTable {
    let pt_physical_frame = Cr3::read();
    let lvl_4_pt_physical_addr = pt_physical_frame.0.start_address().as_u64();

    /* V.addr = P.Addr + offset (setup by the bootloader when paging enabled) */
    let v_addr = VirtualAddress::new(lvl_4_pt_physical_addr) + MEMORY.lock().offset.unwrap();
    let ptr = v_addr.addr as *mut PageTable;
    unsafe { &mut *ptr }
}

pub fn translate_addr(vaddr: VirtualAddress) {}
