#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(libkernel::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(alloc_error_handler)] 

extern crate alloc;

use libkernel::allocator::LockedHeap;
use libkernel::{serial_println, serial_print};
use libkernel::qemu::{qemu_exit, QemuExitCode};
use core::panic::PanicInfo;
use alloc::alloc::{Layout, alloc};

#[global_allocator]
static mut ALLOCATOR: LockedHeap = LockedHeap::empty();

#[alloc_error_handler]
fn alloc_error_handler(_layout: alloc::alloc::Layout) -> ! {
    serial_println!("OK!");
    qemu_exit(QemuExitCode::Success)
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    unsafe {ALLOCATOR.init(0x60000, 100_000);}
    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    libkernel::test_panic_handler(info)
}

#[test_case]
fn request_unsupported_memory() {
    use core::ptr::null_mut;

    serial_print!("request_unsupported_memory...");
    let layout = Layout::from_size_align(4096, 4);
    let _ptr: *mut u8;

    match layout {
        Ok(lay) => unsafe {
            _ptr = alloc(lay);
        },
        Err(err) => {
            serial_println!("Layout create failed, err = {}", err);
            qemu_exit(QemuExitCode::Failed);
        }
    }

    if _ptr == null_mut() {
        serial_println!("OK!");
        qemu_exit(QemuExitCode::Success);
    }

    serial_println!("FAIL");
    qemu_exit(QemuExitCode::Failed);
}
