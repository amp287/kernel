#![no_main]
#![no_std]

use core::panic::PanicInfo;
use libkernel::serial_println;
use cortex_a::regs;
use register::cpu::RegisterReadOnly;

extern crate alloc;

#[no_mangle]
pub unsafe extern "C" fn kernel_main(r0: u32, r1: u32, atags: u32) -> ! {
    serial_println!("hello World!");
    serial_println!("this is a test");
    serial_println!("r0 arg = {}", r0);
    serial_println!("r1 arg = {}", r1);
    serial_println!("atags arg = {}", atags);

    let el: &'static str = match regs::CurrentEL.read_as_enum(regs::CurrentEL::EL) {
        Some(regs::CurrentEL::EL::Value::EL2) => "Hypervisor El2",
        Some(regs::CurrentEL::EL::Value::EL1) => "Kernel El1",
        Some(regs::CurrentEL::EL::Value::EL0) => "User EL0",
        _ => "Unknown"
    };

    serial_println!("Exception Level: {}", el);

    loop {}
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    serial_println!("PANIC!, {}", _panic);
    loop {}
}
