//! Global Descriptor Table module

use lazy_static::lazy_static;
use x86_64::VirtAddr;
use x86_64::instructions::segmentation::{CS, Segment};
use x86_64::instructions::tables::load_tss;
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;

/// Index in the interrupt stack table for the double fault handler
pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

lazy_static! {
    // TSS
    static ref TSS: TaskStateSegment = {
        // Create the TSS
        let mut tss = TaskStateSegment::new();

        // TODO: Use a proper stack allocator and add a stack guard
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            // Use a static mut array as stack storage
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(&raw const STACK);

            // Return the top of the stack
            stack_start + STACK_SIZE.try_into().unwrap()
        };

        tss
    };
}

struct Selectors {
    cs_sel: SegmentSelector,
    tss_sel: SegmentSelector,
}

lazy_static! {
    // GDT
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        // Create the GDT and the selectors
        let mut gdt = GlobalDescriptorTable::new();
        let cs_sel = gdt.append(Descriptor::kernel_code_segment());
        let tss_sel = gdt.append(Descriptor::tss_segment(&TSS));

        // Add the kernel code and TSS segments
        gdt.append(Descriptor::kernel_code_segment());
        gdt.append(Descriptor::tss_segment(&TSS));

        (gdt, Selectors { cs_sel, tss_sel })
    };
}

/// Load GDT, CS, and TSS in the CPU
pub fn init() {
    GDT.0.load();

    unsafe {
        CS::set_reg(GDT.1.cs_sel);
        load_tss(GDT.1.tss_sel);
    }
}
