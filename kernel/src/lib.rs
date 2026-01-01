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
        interrupts::interrupts::PICS.lock().initialize();
        // Unmask ONLY the timer (IRQ0)
        interrupts::interrupts::PICS.lock().write_masks(0xFE, 0xFF);
    }
    x86_64::instructions::interrupts::enable();
}
