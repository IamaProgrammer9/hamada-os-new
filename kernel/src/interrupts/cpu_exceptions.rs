// use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

// pub extern "x86-interrupt" fn breakpoint_handler(
//     stack_frame: InterruptStackFrame)
// {
//     serial_println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
// }

// pub extern "x86-interrupt" fn double_fault_handler(
//     stack_frame: InterruptStackFrame, _error_code: u64) -> !
// {
//     panic!("EXCEPTION: DOUBLE FAILURE\n{:#?}", stack_frame)
// }