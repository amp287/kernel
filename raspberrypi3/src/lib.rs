#![no_std]
#![cfg_attr(test, no_main)]
#![feature(asm)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(alloc_error_handler)] 

extern crate alloc;

pub mod print;
pub mod uart;
pub mod mailbox;
pub mod qemu;
pub mod allocator;

use core::panic::PanicInfo;
use qemu::{qemu_exit, QemuExitCode};

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}

pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
  
    qemu_exit(QemuExitCode::Success);
}

#[cfg(test)]
#[no_mangle]
pub unsafe extern "C" fn kernel_main() -> ! {
    serial_println!("kernel_main test");
    test_main();
    loop {}
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    qemu_exit(QemuExitCode::Failed);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}