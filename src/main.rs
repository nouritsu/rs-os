#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;

    vga_buffer::WRITER
        .lock()
        .write_str("Hello Thread-Safe Mutability!")
        .unwrap();

    write!(
        vga_buffer::WRITER.lock(),
        "Here are some numbers: {} and {}",
        42.0,
        3.14159
    )
    .unwrap();

    loop {}
}

/* Function called on panic */
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
