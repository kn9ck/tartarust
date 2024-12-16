#![no_std] // dont link rust standard lib
#![no_main] // disable main entry point

mod vga_buffer;

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

//new entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();

    loop {}
}

//called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
