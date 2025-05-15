//! Interrupts module - interrupt handlers are defined here

use lazy_static::lazy_static;
use pic8259::ChainedPics;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use crate::{gdt, print, println};

/// PIC1 interrupt offset
const PIC1_OFFSET: u8 = 32;

/// PIC2 interrupt offset
const PIC2_OFFSET: u8 = PIC1_OFFSET + 8;

/// Chained PICs
static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC1_OFFSET, PIC2_OFFSET) });

/// Initialize the chained PICs
pub fn init_pics() {
    unsafe { PICS.lock().initialize() };
}

/// Interrupt variants
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    /// Timer interrupt
    Timer = PIC1_OFFSET,
    /// Keyboard interrupt
    Keyboard,
}

impl InterruptIndex {
    /// Return `InterruptIndex` as a `u8`
    const fn as_u8(self) -> u8 {
        self as u8
    }
}

lazy_static! {
    /// IDT
    static ref IDT: InterruptDescriptorTable = {
        // Create the IDT
        let mut idt = InterruptDescriptorTable::new();

        // Set handler functions
        unsafe {
            // Use a dedicated stack for the double fault handler
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt[InterruptIndex::Timer.as_u8()].set_handler_fn(timer_interrupt_handler);
        idt[InterruptIndex::Keyboard.as_u8()].set_handler_fn(keyboard_interrupt_handler);

        idt
    };
}

/// Load the IDT in the CPU
pub fn init_idt() {
    IDT.load();
}

/// Double fault handler
extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

/// Breakpoint exception handler
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

/// Timer interrupt handler
extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    print!(".");

    // Notify the end of interrupt
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

/// Keyboard interrupt handler
extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    const KEYBOARD_DATA_PORT: u16 = 0x60;
    let mut port = x86_64::instructions::port::Port::<u8>::new(KEYBOARD_DATA_PORT);

    let key = match unsafe { port.read() } {
        0x02 => Some('1'),
        0x03 => Some('2'),
        0x04 => Some('3'),
        0x05 => Some('4'),
        0x06 => Some('5'),
        0x07 => Some('6'),
        0x08 => Some('7'),
        0x09 => Some('8'),
        0x0a => Some('9'),
        0x0b => Some('0'),
        _ => None,
    };
    if let Some(key) = key {
        print!("{key}");
    }

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}

#[cfg(test)]
mod tests {
    #[test_case]
    fn test_breakpoint_exception() {
        // invoke a breakpoint exception
        x86_64::instructions::interrupts::int3();
    }
}
