#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(abi_x86_interrupt)]

use bootloader_api::{BootInfo, entry_point};
use x86_64::instructions::hlt;
use core::fmt::Write;
mod graphics; // declares that there is a graphics module
use crate::graphics::graphics_engine::draw_rect;
mod interrupts;
pub mod serial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) -> ! {
    use x86_64::instructions::{nop, port::Port};

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }

    loop {
        nop();
    }
}

pub fn serial() -> uart_16550::SerialPort {
    let mut port = unsafe { uart_16550::SerialPort::new(0x3F8) };
    port.init();
    port
}

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    let fb = boot_info.framebuffer.as_mut().expect("No framebuffer");
    let info = fb.info();
    let buffer = fb.buffer_mut();

    // kernel::interrupts::interrupts::init_idt();
    serial_println!("Hello World{}", "!");

    // Clear screen (black)
    for byte in buffer.iter_mut() {
        *byte = 0;
    };

    draw_rect(
        buffer,
        info,
        100, 100,     // x, y
        400, 200,     // width, height
        (0, 255, 0),  // green
    );

    loop {
        hlt();
    }

    // exit_qemu(QemuExitCode::Success);
}

/// This function is called on panic.
#[panic_handler]
#[cfg(not(test))]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let _ = writeln!(serial(), "PANIC: {info}");
    exit_qemu(QemuExitCode::Failed);
}
