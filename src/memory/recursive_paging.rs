/* Probably won't use recursive paging for my OS
 * But, these methods are here just in case I decide to change it
 * Just because recursive paging is an elegant idea, and I like it
 */

/*
 * Get vaddr to access all the different level(1..4) page tables
 * based on recursive paging idea
 */

#[allow(dead_code)]
pub struct PageTableAddrs {
    pub level_4: *const u8,
    pub level_3: *const u8,
    pub level_2: *const u8,
    pub level_1: *const u8,
}

#[allow(dead_code)]
pub fn get_page_table_vaddr(vaddr: usize) -> PageTableAddrs {
    let lvl_4_index = vaddr >> 39 & 0b111111111;
    let lvl_3_index = vaddr >> 30 & 0b111111111;
    let lvl_2_index = vaddr >> 21 & 0b111111111;

    // Assumption: Level 4 page table's last entry(index 511) is the recursive entry i.e. has the PFN
    // for the lvl 4 page table itself
    let recursive_index = 0o777;

    let sign = 0xFFFF << 48;

    let level_4 = (sign
        | (recursive_index << 39)
        | (recursive_index << 30)
        | (recursive_index << 21)
        | (recursive_index << 12)) as *const u8;

    let level_3 = (sign
        | (recursive_index << 39)
        | (recursive_index << 30)
        | (recursive_index << 21)
        | lvl_4_index << 12) as *const u8;

    let level_2 = (sign
        | (recursive_index << 39)
        | (recursive_index << 30)
        | lvl_4_index << 21
        | lvl_3_index << 12) as *const u8;

    let level_1 = (sign
        | (recursive_index << 39)
        | (lvl_4_index << 30)
        | (lvl_3_index << 21)
        | (lvl_2_index << 12)) as *const u8;

    PageTableAddrs {
        level_4,
        level_3,
        level_2,
        level_1,
    }
}
