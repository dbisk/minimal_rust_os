// some attributes to allow this to be freestanding (bare-metal bootable)
#![no_std] // no standard library
#![no_main] // no main function (program-level entry point)

mod vga_buffer;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {

    vga_buffer::vga_text_mode_test(vga_buffer::Color::Yellow, vga_buffer::Color::Black);

    // loop indefinitely
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

