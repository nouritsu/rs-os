#![no_std]
#![no_main]

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";
const VGA_BUF_ADDR: usize = 0xb8000;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = VGA_BUF_ADDR as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

/* Function called on panic */
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
