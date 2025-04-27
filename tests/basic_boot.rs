//! Basic boot integration test

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}

#[unsafe(no_mangle)] // don't mangle the name of this function
extern "C" fn _start() -> ! {
    test_main();

    #[allow(clippy::empty_loop)]
    loop {}
}
