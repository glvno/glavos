// in src/main.rs

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(glavos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use glavos::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    glavos::init();
    x86_64::instructions::interrupts::int3();
    fn stack_overflow() {
        stack_overflow();
    }
    stack_overflow();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    glavos::test_panic_handler(info)
}
