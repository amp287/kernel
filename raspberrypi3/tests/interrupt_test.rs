#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(libkernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use core::panic::PanicInfo;
use libkernel::serial_println;

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    serial_println!("--------------- interrupt_test --------------- ");
    unsafe { libkernel::interrupt::interrupt_init(); }
    test_main();
    serial_println!("--------------- Success --------------- ");
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    libkernel::test_panic_handler(info)
}
