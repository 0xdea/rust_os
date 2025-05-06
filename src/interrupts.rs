//! Interrupts module

use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use crate::println;

lazy_static! {
    /// Global IDT
    static ref IDT: InterruptDescriptorTable = {
        // Create the IDT
        let mut idt = InterruptDescriptorTable::new();

        // Set handler functions
        idt.breakpoint.set_handler_fn(breakpoint_handler);

        idt
    };
}

/// Load the IDT in the CPU
pub fn init_idt() {
    IDT.load();
}

/// Breakpoint interrupt handler
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

#[cfg(test)]
mod tests {
    #[test_case]
    fn test_breakpoint_exception() {
        // invoke a breakpoint interrupt
        x86_64::instructions::interrupts::int3();
    }
}
