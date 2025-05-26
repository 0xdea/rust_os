//! Interrupts module - interrupt handlers are defined here

use lazy_static::lazy_static;
use pc_keyboard::{DecodedKey, HandleControl, Keyboard, ScancodeSet1, layouts};
use pic8259::ChainedPics;
use spin::Mutex;
use x86_64::instructions::port::Port;
use x86_64::registers::control::Cr2;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

use crate::{gdt, hlt_loop, print, println};

/// PIC1 interrupt offset
const PIC1_OFFSET: u8 = 32;

/// PIC2 interrupt offset
const PIC2_OFFSET: u8 = PIC1_OFFSET + 8;

/// Chained Programmable Interrupt Controllers
static PICS: Mutex<ChainedPics> = Mutex::new(unsafe { ChainedPics::new(PIC1_OFFSET, PIC2_OFFSET) });

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
    /// Interrupt Descriptor Table
    static ref IDT: InterruptDescriptorTable = {
        // Create the IDT
        let mut idt = InterruptDescriptorTable::new();

        // Set handler functions
        unsafe {
            // Use a dedicated stack for the double fault handler
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt.page_fault.set_handler_fn(page_fault_handler);
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
    panic!("EXCEPTION: DOUBLE FAULT\n{stack_frame:#?}");
}

// Page fault handler
extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    println!("EXCEPTION: PAGE FAULT");
    println!("Accessed Address: {:?}", Cr2::read());
    println!("Error Code: {error_code:?}");
    println!("{stack_frame:#?}");
    hlt_loop();
}

/// Breakpoint exception handler
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{stack_frame:#?}");
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
    let mut port = Port::<u8>::new(KEYBOARD_DATA_PORT);

    // Create a static `Keyboard` protected by a mutex
    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
            Mutex::new(Keyboard::new(
                ScancodeSet1::new(),
                layouts::Us104Key,
                // Handle the Ctrl key like a normal key
                HandleControl::Ignore
            ));
    }

    // Lock the mutex
    let mut keyboard = KEYBOARD.lock();

    // Read and process the scancode
    let scancode = unsafe { port.read() };
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode)
        && let Some(key) = keyboard.process_keyevent(key_event)
    {
        match key {
            DecodedKey::Unicode(character) => print!("{character}"),
            DecodedKey::RawKey(keycode) => print!("{keycode:?}"),
        }
    }

    // Notify the end of interrupt
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}

#[cfg(test)]
mod tests {
    use x86_64::instructions::interrupts::int3;

    #[test_case]
    fn test_breakpoint_exception() {
        // invoke a breakpoint exception
        int3();
    }
}
