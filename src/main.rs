//! main

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use rust_os::println;

/// Panic handler
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

/// Panic handler for tests
#[cfg(test)]
#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}

/// Program's entry point
//noinspection RsUnresolvedPath
#[unsafe(no_mangle)]
#[allow(clippy::missing_panics_doc)] // Writes to the VGA buffer never fail
extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    // Initialize the OS
    rust_os::init();

    // trigger a page fault
    unsafe {
        *(0xdeadbeef as *mut u8) = 42;
    };

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    #[allow(clippy::empty_loop)]
    loop {}
}

#[cfg(test)]
mod tests {
    #[test_case]
    #[allow(clippy::eq_op)]
    fn trivial_assertion() {
        assert_eq!(1, 1);
    }
}
