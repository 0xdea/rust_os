//! Serial module

use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;

lazy_static! {
    /// Global serial port
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

/// Print to the host through the serial interface
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::print_helper(format_args!($($arg)*));
    };
}

/// Print to the host through the serial interface, with a newline
#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}

/// Helper function for the print macros
#[doc(hidden)]
#[allow(dead_code)]
pub fn print_helper(args: core::fmt::Arguments) {
    use core::fmt::Write;
    SERIAL1
        .lock()
        .write_fmt(args)
        .expect("Printing to serial failed");
}
