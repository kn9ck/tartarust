#![no_std] // dont link rust standard lib
#![no_main] // disable main entry point

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

//new entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

//called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
