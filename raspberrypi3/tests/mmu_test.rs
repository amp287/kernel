#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(libkernel::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(alloc_error_handler)] 

extern crate alloc;

use libkernel::mmu::*;
use libkernel::{serial_println, serial_print};
use libkernel::qemu::{QemuExitCode, qemu_exit};
use core::panic::PanicInfo;
use libkernel::allocator::LockedHeap;
use core::convert::TryInto;

#[global_allocator]
static mut ALLOCATOR: LockedHeap = LockedHeap::empty();

#[alloc_error_handler]
fn alloc_error_handler(_layout: alloc::alloc::Layout) -> ! {
    serial_println!("[Failure] Shouldnt be any memory allocation!");
    qemu_exit(QemuExitCode::Failed)
}

#[no_mangle]
static mut LVL0TABLE: TranslationTable = TABLE_INIT;
static mut LVL1TABLE: TranslationTable = TABLE_INIT; 
static mut LVL2TABLE: TranslationTable = TABLE_INIT; 
static mut LVL3TABLE: TranslationTable = TABLE_INIT; 


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
fn table_descriptor() {
    serial_print!("table_descriptor...");

    let attribtues = TableDescriptorAttributes{
        non_secure: true,
        access_permissions: AccessPermissions::ReadOnlyHigherEL, 
        user_exec_never: true,
        priv_exec_never: false,
    };

    let descriptor = TableDescriptor::new(PhysicalAddress::new(0xF0F0), GranuleSize::_4KB, attribtues);

    assert_eq!(descriptor.to_u64(), 0xD000_0000_0000_F003);

    serial_println!("OK!");
}

#[test_case]
fn block_descriptor() {
    serial_print!("block_descriptor...");

    let attributes = BlockDescriptorAttributes{
        page_base_hw_attr: 0,
        user_exec_never: false, 
        priv_exec_never: false,
        contigous: false,
        dirty_bit: false,
        block_translation_entry: false,
        not_global: false,
        access_flag: false,
        shareability: Shareability::InnerShareable,
        access_permissions: AccessPermissions::ReadWriteAllEL,
        non_secure: true,
        attribute_index: AttributeIndex::Zero,
    };

    let descriptor = BlockDescriptor::new(PhysicalAddress::new(0xA4D_F0F0), GranuleSize::_4KB, attributes);

    assert_eq!(descriptor.to_u64(), 0xA4D_F363);
    serial_println!("OK!");
}

#[test_case]
#[no_mangle]
fn simple_write() {
    use cortex_a::regs::ID_AA64MMFR0_EL1;
    use register::cpu::RegisterReadOnly;

    serial_print!("simple_write...");

    if !ID_AA64MMFR0_EL1.matches_all(ID_AA64MMFR0_EL1::TGran4::Supported) {
        panic!("4kb not supported!");
    }
 
    let mut block_entry: BlockDescriptor;
    let mut phys_addr: PhysicalAddress;
    let mut table_entry: TableDescriptor;

    let mut attributes = TableDescriptorAttributes{
        non_secure: true,
        access_permissions: AccessPermissions::ReadWriteAllEL, 
        user_exec_never: false,
        priv_exec_never: false,
    };

    let block_attrib = BlockDescriptorAttributes{
        page_base_hw_attr: 0,
        user_exec_never: false, 
        priv_exec_never: false,
        contigous: false,
        dirty_bit: false,
        block_translation_entry: false,
        not_global: false,
        access_flag: true,
        shareability: Shareability::InnerShareable,
        access_permissions: AccessPermissions::ReadWriteAllEL,
        non_secure: true,
        attribute_index: AttributeIndex::Zero,
    };

    unsafe {

        serial_println!("level 0 table: {:p}", &LVL0TABLE);
        serial_println!("level 1 table: {:p}", &LVL1TABLE);
        serial_println!("level 2 table: {:p}", &LVL2TABLE);
        serial_println!("level 3 table: {:p}", &LVL3TABLE);

        phys_addr = PhysicalAddress::from_reference(&LVL1TABLE);
        table_entry = TableDescriptor::new(phys_addr, GranuleSize::_4KB, attributes);

        LVL0TABLE.set_entry(0, TableEntry::Table(table_entry));
    
        phys_addr = PhysicalAddress::from_reference(&LVL2TABLE);
        table_entry = TableDescriptor::new(phys_addr, GranuleSize::_4KB, attributes);
        LVL1TABLE.set_entry(0, TableEntry::Table(table_entry));
        
        phys_addr = PhysicalAddress::from_reference(&LVL3TABLE);
        table_entry = TableDescriptor::new(phys_addr, GranuleSize::_4KB, attributes);
        LVL2TABLE.set_entry(0, TableEntry::Table(table_entry));

        // for uart
        table_entry = TableDescriptor::new(phys_addr, GranuleSize::_4KB, attributes);
        LVL2TABLE.set_entry(505, TableEntry::Table(table_entry));
    }

    // uart physical address
    block_entry = BlockDescriptor::new(PhysicalAddress::new(0x3F201000), GranuleSize::_4KB, block_attrib);
    unsafe {
        LVL3TABLE.set_entry(1, TableEntry::Block(block_entry));
    }

    // set kernel code 1 to 1 mapping (36KB)
    for (index, address) in (0x80000_u64..0xE2000_u64).step_by(0x1000).enumerate() {
        block_entry = BlockDescriptor::new(PhysicalAddress::new(address.try_into().unwrap()), GranuleSize::_4KB, block_attrib);

        serial_println!("Setting index: {}, address: {:X}, block_entry: {}", index + 128, address, block_entry);
        
        unsafe { 
            LVL3TABLE.set_entry((index + 128).try_into().unwrap(), TableEntry::Block(block_entry));
        }
    } 

    // 8KB of stack 
    for (index, address) in (0x7D000_u64..0x80000_u64).step_by(0x1000).enumerate() {
        block_entry = BlockDescriptor::new(PhysicalAddress::new(address.try_into().unwrap()), GranuleSize::_4KB, block_attrib);

        serial_println!("Setting index: {}", index + 125);

        unsafe { 
            LVL3TABLE.set_entry((index + 125).try_into().unwrap(), TableEntry::Block(block_entry));
        }
    }

    let virt_addr = VirtualAddress::new(0x8A000);
    phys_addr = PhysicalAddress::new(0x7A000);

    // check to see if this physical address is ok to be used (as in its not a register and gpu doesnt change here).
    block_entry = BlockDescriptor::new(phys_addr, GranuleSize::_4KB, block_attrib);

    // creating mapping for this location 0x8A000 (virt) -> 0x1000 (physical)
    // 0x8A000:
    // level 0 index: 0
    // level 1 index: 0
    // levl 2 index: 0
    // level 3 index: 138
    unsafe {

        match LVL0TABLE.get_entry(0) {
            TableEntry::Table(table) => serial_println!("level 0 entry 0: {:?}", table),
            _ => panic!("level 0 table entry 0 is not a table entry!"),
        };

        match LVL1TABLE.get_entry(0) {
            TableEntry::Table(table) => serial_println!("level 1 entry 0: {:?}", table),
            _ => panic!("level 1 table entry 0 is not a table entry!"),
        };

        match LVL2TABLE.get_entry(0) {
            TableEntry::Table(table) => serial_println!("level 2 entry 0: {:?}", table),
            _ => panic!("level 2 table entry 0 is not a table entry!"),
        };

        LVL3TABLE.set_entry(138, TableEntry::Block(block_entry));

        enable_mmu(&LVL0TABLE);
        serial_println!("mmu enabled!");
        let mut ptr: u64 = virt_addr.into();
    
        core::ptr::write_volatile::<u64>(ptr as *mut u64, 0xF0F0_F0F0_F0F0_F0F0);
    
        disable_mmu();
        ptr = phys_addr.into();
        assert_eq!(core::ptr::read_volatile::<u64>(ptr as *const u64), 0xF0F0_F0F0_F0F0_F0F0);
    }
    serial_println!("OK!");
}