#![no_std] // dont link rust standard lib
#![no_main] // disable main entry point
#![feature(custom_test_frameworks)]
#![test_runner(tartarust::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use tartarust::println;

//new entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    tartarust::init();

    #[cfg(test)]
    test_main();

    println!("no crash!");
    loop {}
}

//called on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    tartarust::test_panic_handler(info)
}
