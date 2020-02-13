#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(libkernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use libkernel::allocator::LockedHeap;
use libkernel::serial_println;
use core::panic::PanicInfo;


#[global_allocator]
static mut ALLOCATOR: LockedHeap = LockedHeap::empty();

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    unsafe {ALLOCATOR.init(0x20000, 64*100);}
    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    libkernel::test_panic_handler(info)
}

#[test_case]
fn allocate_test() {
    use alloc::boxed::Box;
    serial_println!("testing allocator...");
    
    let heap_value = Box::new(41);
    assert_eq!(*heap_value, 41);

    serial_println!("[ok]");
}