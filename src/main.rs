#![no_std]
#![no_main]

pub mod gpu;

use gpu::vga::VGA;
use core::panic::PanicInfo;

/// This function is called on kernel panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut vga = VGA::init();
    vga.println(b"Welcom to SecOS \n WARNING: THIS IS ONLY A PROTOTYPE");
    //vga.println(b"WWWW");


    loop {
        
    }
}
