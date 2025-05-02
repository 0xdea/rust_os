//! Integration test for tests that should panic

#![no_std]
#![no_main]

use core::panic::PanicInfo;

use rust_os::{QemuExitCode, exit_qemu, serial_print, serial_println};

/// Panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);

    loop {}
}

/// Integration test entry point
//noinspection RsUnresolvedPath
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    should_fail();
    serial_println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failure);

    #[allow(clippy::empty_loop)]
    loop {}
}

fn should_fail() {
    serial_print!("should_panic::should_fail... ");
    assert_eq!(0, 1);
}
