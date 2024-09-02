// BOOTLOADER OF STAROS
// AUTHOR: KAZOOKI123
// LICENSED BY MIT

#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga::print_string("Hello, StarOS! Welcome user!", vga::Color::LightGreen);
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}