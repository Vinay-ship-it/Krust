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

     use x86_64::registers::control::Cr3;

    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page table at: {:?}", level_4_page_table.start_address());

    
    #[cfg(test)]
    test_main();

    println!("it did not krash");
    
    krust::hlt_loop();

    
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    krust::hlt_loop();

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

//run-args = ["-display", "none", "-vnc", "0.0.0.0:0"]
