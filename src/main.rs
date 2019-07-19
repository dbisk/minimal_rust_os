// some attributes to allow this to be freestanding (bare-metal bootable)
#![no_std] // no standard library
#![no_main] // no main function (program-level entry point)

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

