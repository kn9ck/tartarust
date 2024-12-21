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

    use x86_64::registers::control::Cr3;

    let (level_4_page_table, _) = Cr3::read();
    println!(
        "Level 4 page table at: {:?}",
        level_4_page_table.start_address()
    );

    #[cfg(test)]
    test_main();

    println!("no crash!");
    tartarust::hlt_loop();
}

//called on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    tartarust::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    tartarust::test_panic_handler(info)
}
