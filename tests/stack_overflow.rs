//! Integration test for stack overflow

#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

use lazy_static::lazy_static;
use rust_os::{QemuExitCode, exit_qemu, serial_print, serial_println};
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

lazy_static! {
    /// IDT for testing
    static ref TEST_IDT: InterruptDescriptorTable = {
        // Create the IDT
        let mut idt = InterruptDescriptorTable::new();

        // Set the double fault handler function
        unsafe {
            idt.double_fault
                .set_handler_fn(test_double_fault_handler)
                .set_stack_index(rust_os::gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt
    };
}

/// Load the test IDT in the CPU
pub fn init_test_idt() {
    TEST_IDT.load();
}

/// Panic handler
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}

/// Integration test entry point
#[unsafe(no_mangle)]
#[allow(clippy::missing_panics_doc)]
pub extern "C" fn _start() -> ! {
    serial_print!("stack_overflow::stack_overflow... ");

    // Initialize the OS with a custom IDT
    rust_os::gdt::init();
    init_test_idt();

    // Trigger a kernel stack overflow
    stack_overflow();

    panic!("Execution continued after stack overflow");
}

/// Helper function to trigger a stack overflow
#[allow(unconditional_recursion)]
fn stack_overflow() {
    // For each recursion, the return address is pushed to the stack
    stack_overflow();
    // Prevent tail recursion optimizations
    volatile::Volatile::new(0).read();
}

extern "x86-interrupt" fn test_double_fault_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);

    #[allow(clippy::empty_loop)]
    loop {}
}
