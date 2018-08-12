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
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
            .set_stack_index(mora_os::gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn double_fault_handler (stack_frame: &mut ExceptionStackFrame,
                                                _error_code: u64) {
    println_serial!("ok");
    unsafe { exit_qemu(); }
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
    mora_os::gdt::init();
    init_idt();

    fn stack_overflow() {
        stack_overflow(); // for each recursion, the return address is pushed
    }

    stack_overflow();
    unsafe { exit_qemu(); }
    loop {}
}
