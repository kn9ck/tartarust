#![no_std] // dont link rust standard lib
#![no_main] // disable main entry point

use core::panic::PanicInfo;

//called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

//new entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop{}
}
