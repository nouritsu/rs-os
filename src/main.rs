#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello println! Here is a number {}", "45");

    loop {}
}

/* Function called on panic */
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
