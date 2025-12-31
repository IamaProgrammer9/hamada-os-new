use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

pub extern "x86-interrupt" fn timer_interrupt_handler(
    _stack_frame: InterruptStackFrame
) {
    unsafe {
        crate::interrupts::interrupts::PICS
            .lock()
            .notify_end_of_interrupt(
                crate::interrupts::interrupts::InterruptIndex::Timer.as_u8()
            );
    }
}