#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// Panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Program's entry point
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    loop {}
}
