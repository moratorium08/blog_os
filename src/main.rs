#![feature(panic_implementation)]
#![feature(exclusive_range_pattern)]
#![no_std]
#![no_main]

extern crate bootloader_precompiled;
extern crate spin;
extern crate volatile;
#[macro_use]
extern crate lazy_static;

#[macro_use]
mod vga_buffer;

use core::panic::PanicInfo;

use vga_buffer::{Buffer, Writer, Color, ColorCode};

static HELLO: &[u8] = b"Hello World!";

#[panic_implementation]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    println!("Hello World{}", "!");
    panic!("hoge");

    loop {}
}
