#![allow(dead_code)]
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::interrupts::cpu_exceptions;
use crate::interrupts::timer;
use crate::interrupts::keyboard;
use lazy_static::lazy_static;
use crate::serial_println;
use crate::gdt;
use x86_64::instructions::interrupts;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(cpu_exceptions::breakpoint_handler);
        unsafe {
            idt.double_fault.set_handler_fn(cpu_exceptions::double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX); // new
        }
        idt[InterruptIndex::Keyboard.as_usize()]
            .set_handler_fn(keyboard::keyboard_interrupt_handler);
        idt[InterruptIndex::Timer.as_usize()]
            .set_handler_fn(timer::timer_interrupt_handler);

        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

use pic8259::ChainedPics;
use spin;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

impl InterruptIndex {
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}
