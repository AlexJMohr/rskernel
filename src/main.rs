#![no_std]
#![no_main]

use core::panic::PanicInfo;
mod vga_buffer;

/// Panic handler, prints to the VGA buffer
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

/// Kernel entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    panic!("panic test");
}
