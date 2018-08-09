#![feature(panic_implementation)]
#![feature(exclusive_range_pattern)]
#![no_std]
#![no_main]

extern crate bootloader_precompiled;
extern crate volatile;

mod vga_buffer;

use core::panic::PanicInfo;

use vga_buffer::{Buffer, Writer, Color, ColorCode};

static HELLO: &[u8] = b"Hello World!";

#[panic_implementation]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
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
    writer.write_byte(b'H');
    writer.write_string("ello ");
    writer.write_string("Wörld!");
    write!(writer, "The numebrs {}, {}", 42, 1.0 / 3.0).unwrap();
    writeln!(writer, "hoge");
    writeln!(writer, "fuga");

    loop {}
}
