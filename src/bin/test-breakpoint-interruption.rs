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
use core::sync::atomic::{AtomicUsize, Ordering};

use x86_64::structures::idt::{InterruptDescriptorTable, ExceptionStackFrame};

use mora_os::exit_qemu;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

static BREAKPOINT_HANDLER_CALLED: AtomicUsize = AtomicUsize::new(0);

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler (_stack_frame: &mut ExceptionStackFrame) {
    BREAKPOINT_HANDLER_CALLED.fetch_add(1, Ordering::SeqCst);
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
    init_idt();
    x86_64::instructions::int3();

    match BREAKPOINT_HANDLER_CALLED.load(Ordering::SeqCst) {
        1 => println_serial!("ok"),
        0 => {
            println_serial!("failed");
            println_serial!("Breakpoint handler has not been called yet.");
        },
        n => {
            println_serial!("failed");
            println_serial!("Breakpoint handler has been called {} times.", n);
        },

    }

    unsafe { exit_qemu(); }
    loop {}
}
