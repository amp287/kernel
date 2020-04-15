#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(libkernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use libkernel::allocator::LockedHeap;
use libkernel::{serial_println, serial_print};
use libkernel::qemu::{QemuExitCode, qemu_exit};
use libkernel::ALLOCATOR;
use core::panic::PanicInfo;
use alloc::boxed::Box;

const MEM_SIZE:usize = 64*1000;

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    unsafe {ALLOCATOR.init(0x20000, MEM_SIZE);}
    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    libkernel::test_panic_handler(info)
}

#[test_case]
fn allocate_test() {
    serial_print!("testing allocator...");
    
    let heap_value = Box::new(41);
    assert_eq!(*heap_value, 41);

    serial_println!("[ok]");
}

#[test_case]
fn large_vec() {
    use alloc::vec::Vec;
    serial_print!("large_vec... ");
    let n = 250;
    let mut vec = Vec::new();
    let mut sum:u64 = 0;
    
    for i in 0..n {
        sum += i;
        //serial_println!("Pushing: {}", i);
        vec.push(i);
    }

    // for i in vec.iter() {
    //     serial_println!("contains: {}", i);
    // }

    assert_eq!(sum, (n-1) * n / 2);
    assert_eq!(vec.iter().sum::<u64>(), (n - 1) * n / 2);
    serial_println!("[ok]");
}

#[test_case]
fn many_boxes() {
    serial_print!("many_boxes... ");
    for i in 0..1000 {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
    serial_println!("[ok]");
}

struct Node {
    next: Option<Box<Node>>,
    val: u64,
}

#[test_case] 
fn fill_mem() {
    serial_print!("fill_mem... ");
    const ARRAY_SIZE:usize = (MEM_SIZE / 9) / 16;
    
    let mut byte16: Node = Node{next: None, val: 0};
    let mut iter: &mut Node = &mut byte16;
    
    for i in 0..ARRAY_SIZE {
        iter.val = i as u64;
        iter.next = Some(Box::<Node>::new(Node{next: None, val: 0}));
        iter = iter.next.as_mut().unwrap();
    }

    let mut iter: &Node = &byte16;
    for i in 0..ARRAY_SIZE {
        assert_eq!(iter.val, i as u64);
        iter = iter.next.as_ref().unwrap();
    }
    serial_println!("[ok]");
}