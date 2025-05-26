//! Memory module

use x86_64::registers::control::Cr3;
use x86_64::{VirtAddr, structures::paging::PageTable};

/// Return a mutable reference to the active level 4 table.
///
/// # Safety
/// The caller must guarantee that the complete physical memory is mapped to virtual memory at the
/// specified `physical_memory_offset`. In addition, this function must be only called once to avoid
/// aliasing `&mut` references (which is undefined behavior).
#[must_use]
pub unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    // Read the physical frame of the active level 4 table, ignoring CR3 flags
    let (level_4_table_frame, _) = Cr3::read();

    // Get the virtual address where the page table frame is mapped as a mutable raw pointer
    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();
    
    // Return a mutable reference to the pointer
    unsafe { &mut *page_table_ptr }
}
