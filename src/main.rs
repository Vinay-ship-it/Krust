#![no_std]
#![no_main]
mod vga_buffer;

use core::panic::PanicInfo;

static Hello: &[u8] = b"Hello, World!";

#[unsafe(no_mangle)] // disabling mangling to ensure the compiler outputs a function witht the name _start
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();
    loop{}
}

#[panic_handler] //define our panic handler as we are using no_std
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

