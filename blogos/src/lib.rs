#![no_std] // don't link the Rust standard library
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::custom_test::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

pub use crate::custom_test::test_panic_handler;

pub mod vga_buffer;
pub mod serial;
pub mod qemu;
pub mod custom_test;
pub mod interrupts;
pub mod keyboard;

pub fn init() {
    interrupts::init_idt();
}

/// Entry point for `cargo test`
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    test_main();
    loop {}
}

// our existing panic handler
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// our panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
