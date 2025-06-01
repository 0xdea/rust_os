//! Memory module

use x86_64::registers::control::Cr3;
use x86_64::structures::paging::{
    FrameAllocator, Mapper, OffsetPageTable, Page, PageTable, PhysFrame, Size4KiB,
};
use x86_64::{PhysAddr, VirtAddr};

/// A FrameAllocator that always returns `None`.
pub struct EmptyFrameAllocator;

/// # Safety
/// The implementer must guarantee that the allocator yields only unused frames. Our implementation
/// always returns `None`, so this isn't a problem in this case.
unsafe impl FrameAllocator<Size4KiB> for EmptyFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        None
    }
}

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

/// Create an example mapping for the given page to frame `0xb8000`.
///
/// # Safety
/// The caller must ensure that the frame is not already in use, as mapping to the same physical
/// frame twice could result in undefined behavior.
/// FIXME: In our case, we reuse the VGA text buffer frame, so we break the required condition!
///
/// # Panics
/// This function panics if `map_to` fails.
pub fn create_example_mapping(
    page: Page,
    mapper: &mut OffsetPageTable,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) {
    use x86_64::structures::paging::PageTableFlags as Flags;

    let frame = PhysFrame::containing_address(PhysAddr::new(0xb8000));
    let flags = Flags::PRESENT | Flags::WRITABLE;

    // Create a new mapping in the page table and flush the page from the TLB
    let map_to_result = unsafe {
        // FIXME: this is not safe, we do it only for testing
        mapper.map_to(page, frame, flags, frame_allocator)
    };
    map_to_result.expect("map_to failed").flush();
}
