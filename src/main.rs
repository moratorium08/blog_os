#![feature(panic_implementation)]
#![no_std]
#![feature(exclusive_range_pattern)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]
#![feature(abi_x86_interrupt)]

#[macro_use]
extern crate mora_os;
extern crate x86_64;
#[macro_use]
extern crate lazy_static;

use core::panic::PanicInfo;

use x86_64::structures::idt::{InterruptDescriptorTable, ExceptionStackFrame};

use mora_os::exit_qemu;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler (stack_frame: &mut ExceptionStackFrame) {
    println!("Exception: Breakpoint\n{:#?}", stack_frame);
}

#[cfg(not(test))]
#[panic_implementation]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    println_serial!("Hello Serial: {}", 1);
    init_idt();
    x86_64::instructions::int3();
    println!("Ok");

    unsafe { exit_qemu(); }
    loop {}
}
