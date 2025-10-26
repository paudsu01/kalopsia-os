// Setup Task State Segment

use lazy_static::lazy_static;
use x86_64::instructions::segmentation::{Segment, CS};
use x86_64::instructions::tables::load_tss;
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};

mod tss;
pub use tss::DOUBLE_FAULT_STACK_INDEX;

pub struct GDTInfo {
    gdt: GlobalDescriptorTable,
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

lazy_static! {
    pub static ref GDT: GDTInfo = new();
}

fn new() -> GDTInfo {
    let mut gdt = GlobalDescriptorTable::new();
    // Add two segment descriptors to the GDT
    let code_selector = gdt.append(Descriptor::kernel_code_segment());
    let tss_selector = gdt.append(Descriptor::tss_segment(&tss::TSS));
    GDTInfo {
        gdt,
        code_selector,
        tss_selector,
    }
}

pub fn init() {
    // loads the new GDT using the `ldgt` instruction
    GDT.gdt.load();

    // Change the segment register(CS and TR) to have the new appropriate indexes for the new GDT
    unsafe {
        CS::set_reg(GDT.code_selector);
        // uses the `ltr` instruction which also uses the index to the TSS descriptor in the GDT
        load_tss(GDT.tss_selector);
    }
}
