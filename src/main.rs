#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[unsafe(no_mangle)] // disabling mangling to ensure the compiler outputs a function witht the name _start
pub extern "C" fn _start() -> ! {
    loop{}
}

#[panic_handler] //define our panic handler as we are using no_std
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

