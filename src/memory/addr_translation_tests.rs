use crate::memory::{translate_address, VirtualAddress};

#[test_case]
fn vga_address_translation() {
    let vga_vaddr = 0xb8000;
    let vga_paddr = translate_address(VirtualAddress::new(vga_vaddr));
    assert_eq!(vga_vaddr, vga_paddr.unwrap().as_u64(),);
}
