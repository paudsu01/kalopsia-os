use lazy_static::lazy_static;
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;

const STACK_SIZE: u64 = 4096 * 5;
pub const DOUBLE_FAULT_STACK_INDEX: usize = 0;
static mut DOUBLE_FAULT_STACK: [u8; STACK_SIZE as usize] = [0; STACK_SIZE as usize];

lazy_static! {
    pub static ref TSS: TaskStateSegment = init();
}

#[allow(dead_code)]
pub fn init() -> TaskStateSegment {
    let mut tss = TaskStateSegment::new();
    init_ist(&mut tss);
    // init_pst();
    tss
}

// fn to setup the Interrupt Stack Table(IST)
fn init_ist(tss: &mut TaskStateSegment) {
    // add stack pointer so that the CPU can switch to this stack on double faults
    tss.interrupt_stack_table[DOUBLE_FAULT_STACK_INDEX] = {
        let addr = VirtAddr::from_ptr(&raw const DOUBLE_FAULT_STACK);
        addr + STACK_SIZE
    };
}

// fn to setup the Privilege Stack Table(IST)
#[allow(dead_code)]
fn init_pst() {
    unimplemented!();
}
