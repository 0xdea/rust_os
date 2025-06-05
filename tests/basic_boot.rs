//! Basic boot integration test

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use bootloader::{BootInfo, entry_point};
use rust_os::hlt_loop;

entry_point!(main);

/// Panic handler
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}

/// Integration test entry point
//noinspection RsUnresolvedPath
fn main(_boot_info: &'static BootInfo) -> ! {
    test_main();
    hlt_loop();
}

#[cfg(test)]
mod tests {
    use rust_os::println;

    #[test_case]
    fn test_println() {
        println!("test_println output");
    }
}
