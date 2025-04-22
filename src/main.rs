#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// Program's entry point
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    loop {}
}

/// Panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
