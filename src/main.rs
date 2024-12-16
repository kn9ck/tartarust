#![no_std] // dont link rust standard lib
#![no_main] // disable main entry point

mod vga_buffer;


//new entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    
    loop {}
}

use core::panic::PanicInfo;
//called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
