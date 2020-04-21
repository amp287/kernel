#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(libkernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use libkernel::exception;
use libkernel::serial_println;
use libkernel::qemu;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {

    unsafe { libkernel::interrupt::interrupt_init(); }
    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    libkernel::test_panic_handler(info)
}


#[test_case]
fn get_current_el_test() {
    let el = exception::get_current_exception_level();
    serial_println!("get_current_el_test...");

    serial_println!("EL= {}", el);

    match el {
        exception::ExceptionLevel::EL_1 => serial_println!("OK!"),
        _ => panic!("Incorrect exception level"),
    };
}

#[allow(dead_code)]
fn print_moose() {
    serial_println!("moose");
    qemu::qemu_exit(qemu::QemuExitCode::Success);
}

//#[test_case] not ready
#[allow(dead_code)]
fn move_to_el_0() {
    serial_println!("move_to_el_0...");
    unsafe {
        exception::el1_to_el0(0x1000,print_moose);
    }
    
    panic!("Failed!");
}