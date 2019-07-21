// some attributes to allow this to be freestanding (bare-metal bootable)
#![no_std] // no standard library
#![no_main] // no main function (program-level entry point)

mod vga_writer;

use core::panic::PanicInfo;
use crate::vga_writer::vga_buffer::*;

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {

    // write a string to the screen using the vga_writer
    let mut writer = vga_writer::Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer)}
    };

    writer.write_byte(b'H');

    // loop indefinitely
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

