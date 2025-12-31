#![no_std]
#![feature(abi_x86_interrupt)]

pub mod interrupts;
pub mod serial;
pub mod gdt;
pub mod graphics;

pub fn init() {
    gdt::init();
    interrupts::interrupts::init_idt();
    unsafe {
        use crate::interrupts::interrupts::PICS;

        PICS.lock().initialize();

        // Unmask timer (IRQ0) and keyboard (IRQ1)
        PICS.lock().write_masks(
            0b1111_1100, // master PIC
            0b1111_1111, // slave PIC
        );
    }
    x86_64::instructions::interrupts::enable();
}
