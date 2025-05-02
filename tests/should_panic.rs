//! Integration test for tests that should panic

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use rust_os::{QemuExitCode, exit_qemu, serial_println};

/// Panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);

    loop {}
}

/// Custom test runner for this integration test
pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
        serial_println!("[test did not panic]");
        exit_qemu(QemuExitCode::Failure);
    }
    exit_qemu(QemuExitCode::Success);
}

/// Integration test entry point
//noinspection RsUnresolvedPath
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    test_main();

    #[allow(clippy::empty_loop)]
    loop {}
}

// Only a single test case is supported
#[cfg(test)]
mod tests {
    use rust_os::serial_print;

    #[test_case]
    fn should_fail() {
        serial_print!("should_panic::should_fail...\t");
        assert_eq!(0, 1);
    }
}
