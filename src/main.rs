#![no_std] // don't link Rust standard library
#![no_main] // disable Rust-level entry point

use core::panic::PanicInfo;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function named `_start` by default
    loop {}
}

// this function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

