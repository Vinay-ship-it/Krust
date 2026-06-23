#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(krust::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use krust::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World!");

    krust::init();

    println!("hey");

    #[cfg(test)]
    test_main();

    println!("It did not krash!");

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    krust::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

