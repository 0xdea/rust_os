//! Memory module

use x86_64::registers::control::Cr3;
use x86_64::structures::paging::{PageTable, page_table::FrameError};
use x86_64::{PhysAddr, VirtAddr};

/// Translate the given virtual address to the mapped physical address, or return `None` if the
/// address is not mapped.
///
/// # Safety
/// The caller must guarantee that the complete physical memory is mapped to virtual memory at the
/// specified `physical_memory_offset`.
#[must_use]
pub unsafe fn translate_addr(addr: VirtAddr, physical_memory_offset: VirtAddr) -> Option<PhysAddr> {
    translate_addr_inner(addr, physical_memory_offset)
}

/// Private function that is called by `translate_addr`.
///
/// This function is safe to limit the scope of `unsafe` because Rust treats the whole body of
/// unsafe functions as an unsafe block.
fn translate_addr_inner(addr: VirtAddr, physical_memory_offset: VirtAddr) -> Option<PhysAddr> {
    // Read the physical frame of the active level 4 table, ignoring CR3 flags
    let (level_4_table_frame, _) = Cr3::read();
    let mut frame = level_4_table_frame;

    // Store the indexes of the page table levels
    let table_indexes = [
        addr.p4_index(),
        addr.p3_index(),
        addr.p2_index(),
        addr.p1_index(),
    ];

    // Traverse the multi-level page table
    for &index in &table_indexes {
        // Convert the frame into a page table reference
        let virt = physical_memory_offset + frame.start_address().as_u64();
        let table_ptr: *const PageTable = virt.as_ptr();
        let table = unsafe { &*table_ptr };

        // Read the page table entry and update `frame`
        let entry = &table[index];
        frame = match entry.frame() {
            Ok(frame) => frame,
            Err(FrameError::FrameNotPresent) => return None,
            Err(FrameError::HugeFrame) => panic!("Huge pages not supported"),
        };
    }

    // calculate the physical address by adding the page offset
    Some(frame.start_address() + u64::from(addr.page_offset()))
}
