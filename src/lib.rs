#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::zly_test::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[macro_use]
extern crate lazy_static;

extern crate volatile;

extern crate spin;

pub mod gdt;
pub mod interrupts;
pub mod serial;
pub mod vga_buffer;
pub mod zly_test;

pub fn init() {
    gdt::gdt_init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

/// Entry point for `cargo test`
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    test_main();
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    zly_test::test_panic_handler(info)
}
