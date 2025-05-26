//! main

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![allow(missing_docs)]

use core::panic::PanicInfo;

use bootloader::{BootInfo, entry_point};
use rust_os::memory::active_level_4_table;
use rust_os::{hlt_loop, println};
use x86_64::VirtAddr;
use x86_64::structures::paging::PageTable;

/// Panic handler
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{info}");
    hlt_loop();
}

/// Panic handler for tests
#[cfg(test)]
#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}

entry_point!(kernel_main);

/// Program's entry point
//noinspection RsUnresolvedPath
#[allow(clippy::missing_panics_doc)] // Writes to the VGA buffer never fail
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello World{}", "!");

    // Initialize the OS
    rust_os::init();

    // Get the active level 4 table based on the physical memory offset provided by the bootloader
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let l4_table = unsafe { active_level_4_table(phys_mem_offset) };

    // Iterate over the page table entries and print non-empty entries
    for (i, entry) in l4_table.iter().enumerate() {
        if !entry.is_unused() {
            println!("L4 Entry {i}: {entry:?}");

            // Get the physical address from the entry and convert it
            let phys = entry.frame().unwrap().start_address();
            let virt = phys.as_u64() + boot_info.physical_memory_offset;
            let ptr = VirtAddr::new(virt).as_mut_ptr();
            let l3_table: &PageTable = unsafe { &*ptr };

            // Print non-empty entries of the level 3 table
            for (i, entry) in l3_table.iter().enumerate() {
                if !entry.is_unused() {
                    println!("  L3 Entry {}: {:?}", i, entry);
                }
            }
        }
    }

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    hlt_loop();
}

#[cfg(test)]
mod tests {
    #[test_case]
    #[allow(clippy::eq_op)]
    fn trivial_assertion() {
        assert_eq!(1, 1);
    }
}
