// don't link to Rust standard library
#![no_std]
// disable all rust-level entrypoints
#![no_main]

use core::panic::PanicInfo;

// this function is called on Panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// this function is the entry point, replacing main
#[no_mangle] //don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    loop{}
}

