// some attributes to allow this to be freestanding (bare-metal bootable)
#![no_std] // no standard library
#![no_main] // no main function (program-level entry point)

// some attributes to allow for a custom test framework
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

// change the auto-generated tests function to something callable (not "main")
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {

    vga_buffer::vga_text_mode_test(vga_buffer::Color::Yellow, vga_buffer::Color::Black);

    // run tests if we are in test configuration
    #![cfg(test)]
    test_main();

    // loop indefinitely
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

// runs all the functions in the passed in array of test functions
#![cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Starting first of {} tests", tests.len());
    for test in tests {
        test();
    }

    exit_qemu(QemuExitCode::Success);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11, 
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

