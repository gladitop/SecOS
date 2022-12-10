#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

pub mod gdt;
pub mod gpu;
pub mod interrupts;

use core::panic::PanicInfo;

/// This function is called on kernel panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Kernel panic!");
    println!("Info error: {}", info);

    loop {}
}

pub fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    {
        let mut writer = gpu::vga::WRITER.lock();
        writer.set_background();
    }

    println!("Loading kernel service...");
    init();

    println!("Kernel done!");
    println!("Welcome to OC");
    println!("It's a prototype!");
    println!("Version: 0.1");
    println!("Author: Almaev Damir Maratovich");

    loop {}
}
