// use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
// use crate::interrupts::cpu_exceptions;
// use lazy_static::lazy_static;

// lazy_static! {
//     static ref IDT: InterruptDescriptorTable = {
//         let mut idt = InterruptDescriptorTable::new();
//         idt.breakpoint.set_handler_fn(cpu_exceptions::breakpoint_handler);
//         unsafe {
//             idt.double_fault.set_handler_fn(cpu_exceptions::double_fault_handler)
//                 .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX); // new
//         }
//         idt
//     };
// }

// pub fn init_idt() {
//     IDT.load();
// }
