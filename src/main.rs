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
static HELLO: &str = "Hello, kernel world!";

/// Panic handler
#[panic_handler]
const fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Program's entry point
#[unsafe(no_mangle)]
#[allow(clippy::missing_panics_doc)] // Writes to the VGA buffer never fail
extern "C" fn _start() -> ! {
    use core::fmt::Write;

    vga_buffer::WRITER.lock().write_str(HELLO).unwrap();
    write!(
        vga_buffer::WRITER.lock(),
        ", some numbers: {} {}",
        42,
        1.337
    )
    .unwrap();

    #[allow(clippy::empty_loop)]
    loop {}
}
