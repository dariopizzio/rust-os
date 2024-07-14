#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

mod vga_buffer;

use core::{fmt::Write, panic::PanicInfo};

use vga_buffer::WRITER;

//static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    WRITER.lock().write_str("Hello World!").unwrap();
    WRITER.lock().new_line();
    write!(WRITER.lock(), "Numbers: {} {}", 10, 10.1).unwrap();

    /* First implementation
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte; // ASCII byte
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb; // color byte
        }
    }*/

    loop {}
}

/// Function called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
