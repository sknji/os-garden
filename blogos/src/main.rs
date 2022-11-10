#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(blogos::custom_test::test_runner)]
#![reexport_test_harness_main = "test_main"]

use blogos::println;
use blogos::test_panic_handler;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}