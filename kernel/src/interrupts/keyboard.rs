use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::interrupts::interrupts::{InterruptIndex, PICS};
use crate::graphics::graphics_engine::draw_rect;

pub extern "x86-interrupt" fn keyboard_interrupt_handler(
    stack_frame: InterruptStackFrame
) {
    use x86_64::instructions::port::Port;

    use uart_16550::SerialPort;
    use core::fmt::Write;

    unsafe {
        let mut serial = SerialPort::new(0x3F8);
        serial.write_str("K\n");
    }

    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}

