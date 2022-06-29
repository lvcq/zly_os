#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(zly_os::zly_test::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use zly_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    zly_os::init();

    // invoke a breakpoint exception

    x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    zly_os::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    zly_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    zly_os::zly_test::test_panic_handler(info)
}
