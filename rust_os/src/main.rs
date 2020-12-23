// don't link to Rust standard library
#![no_std]

// disable all rust-level entrypoints
#![no_main]

// using our own panic handling because we are not using the standard lib
use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

// this function is the entry point, replacing main
// simply prints hello world
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop{}
}

// this function is called on Panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}