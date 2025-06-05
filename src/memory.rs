//! Memory module

use bootloader::bootinfo::{MemoryMap, MemoryRegionType};
use x86_64::registers::control::Cr3;
use x86_64::structures::paging::{
    FrameAllocator, Mapper, OffsetPageTable, Page, PageTable, PhysFrame, Size4KiB,
};
use x86_64::{PhysAddr, VirtAddr};

/// A FrameAllocator that always returns `None`
pub struct EmptyFrameAllocator;

/// # Safety
/// The implementer must guarantee that the allocator yields only unused frames. Our implementation
/// always returns `None`, so this isn't a problem in this case.
unsafe impl FrameAllocator<Size4KiB> for EmptyFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        None
    }
}

/// A FrameAllocator that returns usable frames from the bootloader's memory map
pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryMap,
    next: usize,
}

impl BootInfoFrameAllocator {
    /// Create a FrameAllocator from the passed memory map.
    ///
    /// # Safety
    /// The caller must guarantee that the passed memory map is valid. The main requirement is that all
    /// frames that are marked as `USABLE` in it are really unused.
    #[must_use]
    pub const unsafe fn init(memory_map: &'static MemoryMap) -> Self {
        Self {
            memory_map,
            next: 0,
        }
    }

    /// Return an iterator over the usable frames specified in the memory map
    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        const PAGE_SIZE: usize = 4096;

        // Get usable regions from the memory map
        let regions = self.memory_map.iter();
        let usable_regions = regions.filter(|r| r.region_type == MemoryRegionType::Usable);

        // Map each region to its address range
        let addr_ranges = usable_regions.map(|r| r.range.start_addr()..r.range.end_addr());

        // Transform to an iterator of frame start addresses (the bootloader already page-aligns all
        // usable memory regions, so no alignment code is needed here)
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(PAGE_SIZE));

        // Convert each start address to `PhysFrame` to construct the iterator
        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
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
