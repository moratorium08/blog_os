#![feature(panic_implementation)]
#![no_std]
#![feature(exclusive_range_pattern)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

use core::panic::PanicInfo;
#[macro_use]
extern crate mora_os;

use mora_os::exit_qemu;

#[cfg(not(test))]
#[panic_implementation]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    println_serial!("failed");

    unsafe { exit_qemu(); }
    loop {}
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println_serial!("ok");

    unsafe { exit_qemu(); }
    loop {}
}
