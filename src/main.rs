//! main

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use bootloader::{BootInfo, entry_point};
use rust_os::memory::translate_addr;
use rust_os::{hlt_loop, println};
use x86_64::VirtAddr;

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

    // Get the physical memory offset provided by the bootloader
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    // Translate some addresses
    let addresses = [
        // the identity-mapped vga buffer page
        0xb8000,
        // some code page
        0x0020_1008,
        // some stack page
        0x0100_0020_1a10,
        // virtual address mapped to physical address 0
        boot_info.physical_memory_offset,
    ];
    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = unsafe { translate_addr(virt, phys_mem_offset) };
        println!("{:?} -> {:?}", virt, phys);
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
