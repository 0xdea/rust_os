//! Interrupts module

use lazy_static::lazy_static;
use pic8259::ChainedPics;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use crate::{gdt, println};

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

        idt
    };
}

/// Load the IDT in the CPU
pub fn init_idt() {
    IDT.load();
}

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

#[cfg(test)]
mod tests {
    #[test_case]
    fn test_breakpoint_exception() {
        // invoke a breakpoint exception
        x86_64::instructions::interrupts::int3();
    }
}
