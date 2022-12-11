#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

pub mod gdt;
pub mod gpu;
pub mod interrupts;

use gpu::vga::Color;
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
        writer.set_background(Color::Red);
        //println!("HELL");
        //println!("{:?}", writer.buffer.chars[0][0]);
    }

    println!("Loading kernel service...");
    

    println!("Kernel done!");
    println!("Welcome to OC");
    println!("It's a prototype!");
    println!("Version: 0.1");
    println!("Author: Almaev Damir Maratovich");
    println!("Command mode active!");
    init();

    {
        //let mut writer = gpu::vga::WRITER.lock();
        //println!("{:?}", writer.buffer.chars[1][1].read().ascii_character);
    }


    loop {
        //println!(">")
    }
}
