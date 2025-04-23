#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// String to print
static HELLO: &[u8] = b"Hello World!";

/// Color code for cyan
const COLOR_CYAN: u8 = 0x0b;

/// Panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Program's entry point
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = COLOR_CYAN;
        }
    }

    loop {}
}
