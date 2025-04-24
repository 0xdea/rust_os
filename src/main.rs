//!
//! rust_os - My code for "Writing an OS in Rust" tutorial
//! Copyright (c) 2025 Marco Ivaldi <raptor@0xdeadbeef.info>
//!
//! > "You wanted advanced. We're gonna go advanced."
//! >
//! > -- The Rustonomicon
//!
//! My code for "Writing an OS in Rust", a blog series by [Philipp Oppermann](https://github.com/phil-opp)
//! on OS development using Rust.
//!
//! ## Blog post
//! * *TBA*
//!
//! ## See also
//! * <https://os.phil-opp.com/>
//! * <https://github.com/phil-opp/blog_os>
//!

#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

/// String to print
static HELLO: &[u8] = b"Hello, kernel world!";

/// Color code for cyan
const COLOR_LIGHTCYAN: u8 = 0x0b;

/// Panic handler
#[panic_handler]
const fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Program's entry point
#[unsafe(no_mangle)]
#[allow(clippy::cast_possible_wrap)]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = COLOR_LIGHTCYAN;
        }
    }

    vga_buffer::print_something();

    #[allow(clippy::empty_loop)]
    loop {}
}
