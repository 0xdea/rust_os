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
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

pub mod gdt;
pub mod interrupts;
pub mod serial;
pub mod vga_buffer;

/// Something that can be tested
pub trait Testable {
    /// Run the `Testable`
    fn run(&self);
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        // Hack to print the function name
        serial_print!("{}... ", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

/// Custom test runner
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

/// Panic handler helper for tests
pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failure);
    hlt_loop();
}

/// Possible qemu exit codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    /// Success exit code
    Success = 0x10,
    /// Failure exit code
    Failure = 0x11,
}

/// Exit qemu using the `isa-debug-exit` device
pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

/// Energy-efficient infinite loop
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

/// Initialize various aspects of the OS
pub fn init() {
    // Load the GDT
    gdt::init();

    // Load the IDT
    interrupts::init_idt();

    // Enable external interrupts
    interrupts::init_pics();
    x86_64::instructions::interrupts::enable();
}

/// Panic handler
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

/// Test mode entry point
//noinspection RsUnresolvedPath
#[cfg(test)]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    init();
    test_main();
    hlt_loop();
}
