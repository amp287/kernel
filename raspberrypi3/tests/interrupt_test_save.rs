#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(asm)]
#![test_runner(libkernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use libkernel::exception;
use libkernel::serial_println;
use libkernel::qemu;
use libkernel::interrupt::{InterruptInfo, InterruptType};
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    serial_println!("--------------- interrupt_test_save --------------- ");
    unsafe { libkernel::interrupt::interrupt_init(); }
    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    libkernel::test_panic_handler(info)
}

#[no_mangle]
pub extern "C" fn interrupt_syn_el_lower_64(info: &InterruptInfo, _exception_from_lower_level: bool, _interrupt_type: InterruptType) {
    for register in info.gpr.iter() {
        assert_eq!(*register, 1);
    }
    serial_println!("--------------- Success --------------- ");
    qemu::qemu_exit(qemu::QemuExitCode::Success);
}

#[allow(dead_code)]
fn el0() {
    unsafe {
        asm!("
            mov x0, #1
            mov x1, #1
            mov x2, #1
            mov x3, #1
            mov x4, #1
            mov x5, #1
            mov x6, #1
            mov x7, #1
            mov x8, #1
            mov x9, #1
            mov x10, #1
            mov x11, #1
            mov x12, #1
            mov x13, #1
            mov x14, #1
            mov x15, #1
            mov x16, #1
            mov x17, #1
            mov x18, #1
            mov x19, #1
            mov x20, #1
            mov x21, #1
            mov x22, #1
            mov x23, #1
            mov x24, #1
            mov x25, #1
            mov x26, #1
            mov x27, #1
            mov x28, #1
            mov x29, #1
            mov x30, #1
            svc #1
        ");
    }
}

#[test_case]
fn service_call() {
    unsafe { exception::el1_to_el0(0x1000, el0); }
}