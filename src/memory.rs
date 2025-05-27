//! Memory module

use x86_64::VirtAddr;
use x86_64::registers::control::Cr3;
use x86_64::structures::paging::{OffsetPageTable, PageTable};

/// Initialize a new OffsetPageTable that uses the given offset to convert virtual to physical
/// addresses.
///
/// # Safety
/// The caller must guarantee that the complete physical memory is mapped to virtual memory at the
/// specified `physical_memory_offset`. In addition, this function must be only called once to avoid
/// aliasing `&mut` references (which is undefined behavior).
#[must_use]
pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    unsafe {
        let level_4_table = active_level_4_table(physical_memory_offset);

        OffsetPageTable::new(level_4_table, physical_memory_offset)
    }
}

/// Private function that returns a mutable reference to the active level 4 table.
///
/// # Safety
/// The caller must guarantee that the complete physical memory is mapped to virtual memory at the
/// specified `physical_memory_offset`. In addition, this function must be only called once to avoid
/// aliasing `&mut` references (which is undefined behavior).
#[must_use]
unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    // Read the physical frame of the active level 4 table, ignoring CR3 flags
    let (level_4_table_frame, _) = Cr3::read();

    // Get the virtual address where the page table frame is mapped as a mutable raw pointer
    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    // Return a mutable reference to the pointer
    unsafe { &mut *page_table_ptr }
}
