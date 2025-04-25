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
#![feature(custom_test_frameworks)]
#![test_runner(crate::tests::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

mod vga_buffer;

/// Panic handler
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

/// Program's entry point
//noinspection RsUnresolvedPath
#[unsafe(no_mangle)]
#[allow(clippy::missing_panics_doc)] // Writes to the VGA buffer never fail
extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    #[allow(clippy::empty_loop)]
    loop {}
}

#[cfg(test)]
mod tests {
    use crate::{print, println};

    pub fn test_runner(tests: &[&dyn Fn()]) {
        println!("Running {} tests", tests.len());
        for test in tests {
            test();
        }
        println!("Ok");
    }

    #[test_case]
    #[allow(clippy::eq_op)]
    fn trivial_assertion() {
        print!("trivial assertion... ");

        assert_eq!(1, 1);
        println!("[ok]");
    }
}
