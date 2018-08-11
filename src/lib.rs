#![no_std]
#![feature(exclusive_range_pattern)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

extern crate bootloader_precompiled;
extern crate spin;
extern crate volatile;
#[macro_use]
extern crate lazy_static;
#[cfg(test)]
extern crate std;
#[cfg(test)]
extern crate array_init;
extern crate uart_16550;
extern crate x86_64;


#[macro_use]
pub mod vga_buffer;
#[macro_use]
pub mod serial;

pub unsafe fn exit_qemu() {
    use x86_64::instructions::port::Port;

    let mut port = Port::<u32>::new(0xf4);
    port.write(0);
}


