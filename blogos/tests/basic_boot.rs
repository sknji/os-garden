#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blogos::custom_test::test_runner)]
#![reexport_test_harness_main = "test_main"]

use blogos::println;
use core::panic::PanicInfo;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[test_case]
fn test_println() {
    println!("test_println output");
}