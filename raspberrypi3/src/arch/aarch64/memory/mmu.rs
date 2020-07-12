
//use std::fmt;
use core::convert::TryInto;
use crate::{set_bit,set_bits,get_bits};

const LEVEL0_START: u8 = 39;
const LEVEL1_START: u8 = 30;
const LEVEL2_START: u8 = 21;
const LEVEL3_START: u8 = 12;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PhysicalAddress (u64);

impl PhysicalAddress {
    pub fn new(addr: u64) -> PhysicalAddress {
        PhysicalAddress(addr)
    }

    pub fn from_reference<T>(item: &T) -> PhysicalAddress {
        let ptr = item as *const _;

        PhysicalAddress(ptr as u64)
    }
}

impl Into<u64> for PhysicalAddress {
    fn into(self) -> u64 {
        self.0
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct VirtualAddress (u64);

impl VirtualAddress {

    pub fn new(addr: u64) -> VirtualAddress {
        VirtualAddress(addr)
    }

    pub fn new_by_index(level0_index: u64, level1_index: u64, level2_index: u64, level3_index: u64, physical_offset: u16) -> VirtualAddress {
        let mut address: u64 = 0;

        if physical_offset > 0xFFF {
            panic!("Virtual Address invalid physical offset");
        }

        address |= level0_index << LEVEL0_START;
        address |= level1_index << LEVEL1_START;
        address |= level2_index << LEVEL2_START;
        address |= level3_index << LEVEL3_START;
        address |= physical_offset as u64;
        VirtualAddress(address)
    }

    pub fn get_level0_index(self) -> u8 {
        ((self.0 >> LEVEL0_START) & 0xF).try_into().unwrap()
    }

    pub fn get_level1_index(self) -> u8 {
        ((self.0 >> LEVEL1_START) & 0xF).try_into().unwrap()
    }

    pub fn get_level2_index(self) -> u8 {
        ((self.0 >> LEVEL2_START) & 0xF).try_into().unwrap()
    }

    pub fn get_level3_index(self) -> u8 {
        ((self.0 >> LEVEL0_START) & 0xF).try_into().unwrap()
    }

    pub fn get_physical_offset(self) -> u16 {
        (self.0 & 0xFFF).try_into().unwrap()
    }

}

impl Into<u64> for VirtualAddress {
    fn into(self) -> u64 {
        self.0
    }
}

/*impl fmt::Debug for VirtualAddress {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("VirtualAddress")
            .field("level 0 index", &format_args!(self.get_level0_index())
            .field("level 1 index", &format_args!(self.get_level1_index())
            .field("level 2 index", &format_args!(self.get_level2_index())
            .field("level 3 index", &format_args!(self.get_level3_index())
            .finish()
    }
}*/

// TODO add support for granules other than 4KB
#[derive(Clone, Copy)]
#[repr(C)]
#[repr(align(65536))]
pub struct TranslationTable ([u64; 512]);

pub const TABLE_INIT: TranslationTable =  TranslationTable([0; 512]);

impl TranslationTable {
    // TODO: Add in support for multiple granules
    pub fn new() -> TranslationTable {
        TranslationTable([0; 512])
    }

    pub fn get_entry(&self, index: usize) -> TableEntry {
        let entry = self.0[index];

        match entry & 3 {
            1 => {
                let block = BlockDescriptor(entry);
                TableEntry::Block(block)
            },
            3 => {
                let table = TableDescriptor(entry);
                TableEntry::Table(table)
            },
            _ => TableEntry::Invalid,
        }
    } 

    pub fn set_entry(&mut self, index: usize, entry: TableEntry) {
            let entry: u64 = match entry {
                TableEntry::Block (block) => block.0,
                TableEntry::Table (table) => table.0,
                TableEntry::Invalid => 0,
            };

            self.0[index] = entry;
    }
}

#[derive(Copy, Clone, Debug)]
pub enum AccessPermissions {
    ReadWriteHigherEL = 0,
    ReadWriteAllEL = 1,
    ReadOnlyHigherEL = 2,
    ReadOnlyAllEL = 3
}

#[derive(Copy, Clone, Debug)]
pub enum Shareability {
    NonShareable = 0,
    Reserved = 1,
    OuterShareable = 2,
    InnerShareable = 3,
}

#[derive(Copy, Clone, Debug)]
pub enum AttributeIndex {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3, 
    Four = 4, 
    Five = 5, 
    Six = 6, 
    Seven = 7,
}

#[derive(Debug)]
pub enum GranuleSize {
    _4KB = 0,
    _16KB = 1,
    _64KB = 2,
}

impl GranuleSize {
    pub fn get_table_output_address_start_bit_and_mask(&self) -> (u8, u64) {
        match self {
            GranuleSize::_64KB =>    (16, 0xFFFFFFFF  << 16),
            GranuleSize::_16KB =>    (14, 0xFFFFFFFFC << 14),
            GranuleSize::_4KB  =>    (12, 0xFFFFFFFFF << 12),
        }
    }
}

#[derive(Debug)]
pub enum TableEntry {
    Block(BlockDescriptor),
    Table(TableDescriptor),
    Invalid,
}

#[derive(Copy, Clone, Debug)]
pub struct BlockDescriptorAttributes {
    // PBHA ignored when ARMv8.2-TTPBHA is not implemented
    pub page_base_hw_attr: u8,
    pub user_exec_never: bool, 
    pub priv_exec_never: bool,
    pub contigous: bool,
    pub dirty_bit: bool,
    pub block_translation_entry: bool,
    pub not_global: bool,
    pub access_flag: bool,
    pub shareability: Shareability,
    pub access_permissions: AccessPermissions,
    pub non_secure: bool,
    pub attribute_index: AttributeIndex,
}

#[derive(Debug)]
pub struct BlockDescriptor (u64);

use core::fmt;

impl fmt::Display for BlockDescriptor {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:X} = [ PBHA:{:X}, UXN:{}, PXN:{}, Contiguous:{}, DBM:{}, nT:{}, nG:{}, AF:{}, SH:{}, AP:{:X}, NS:{}, AttrIndx:{:X}, Address:{:X} ]",
            self.0,
            get_bits!(self.0, 59, 0xF),
            get_bits!(self.0, 54, 0x1),
            get_bits!(self.0, 53, 0x1), 
            get_bits!(self.0, 52, 0x1),
            get_bits!(self.0, 51, 0x1),
            get_bits!(self.0, 16, 0x1), 
            get_bits!(self.0, 11, 0x1),
            get_bits!(self.0, 10, 0x1), 
            get_bits!(self.0, 8, 0x3),
            get_bits!(self.0, 6, 0x3),
            get_bits!(self.0, 5, 0x1),
            get_bits!(self.0, 2, 0x3),
            get_bits!(self.0, 12, 0xFFFF_FFFF_F)   
        )
    }
}

// Only Stage 1 is implemented
impl BlockDescriptor {

    pub fn new(addr: PhysicalAddress, granule_size: GranuleSize, attributes: BlockDescriptorAttributes) -> BlockDescriptor {
        let mut descriptor = BlockDescriptor(3);

        descriptor.set_page_based_hardware_attributes(attributes.page_base_hw_attr);
        descriptor.set_block_translation_entry(attributes.block_translation_entry);
        descriptor.set_contiguous(attributes.contigous);
        descriptor.set_dirty_bit_modifier(attributes.dirty_bit);
        descriptor.set_memory_attributes_index_field(attributes.attribute_index);
        descriptor.set_non_secure(attributes.non_secure);
        descriptor.set_not_global(attributes.not_global);
        descriptor.set_privleged_execute_never(attributes.priv_exec_never);
        descriptor.set_shareability_field(attributes.shareability);
        descriptor.set_user_execute_never(attributes.user_exec_never);
        descriptor.set_access_permissions(attributes.access_permissions);
        descriptor.set_access_flag(attributes.access_flag);
        descriptor.set_output_address(addr, granule_size);

        descriptor
    }

    pub fn get_output_address(&self, _granule_size: GranuleSize) -> PhysicalAddress {
      PhysicalAddress((self.0 >> 12) & 0xF_FFFF_FFFF)
    }

    pub fn is_invalid(&self) -> bool {
        if self.0 & 1 == 1 {
            false
        } else {
            true
        }
    }

    pub fn to_u64(&self) -> u64 {
        self.0
    } 

    pub fn set_output_address(&mut self, address: PhysicalAddress, _granule_size: GranuleSize) {
       //self.0 = (self.0 & !0xFFFF_FFFF_F000) | (address.0 & 0xFFFF_FFFF_F000);
       set_bits!(self.0, 12, address.0 >> 12, 0xFFFF_FFFF_F);
    }

    pub fn set_invalid(&mut self) {
        self.0 = 0;
    }

    // PBHA bits [62:59]
    pub fn set_page_based_hardware_attributes(&mut self, attributes: u8) {
        set_bits!(self.0, 59, attributes as u64, 0b1111);
    }

    // UXNTable or XNTable bit 54
    pub fn set_user_execute_never(&mut self, bit: bool) {
        set_bit!(self.0, 54, bit as u64);
    }

    // PXNTable bit 59
    pub fn set_privleged_execute_never(&mut self, bit: bool) {
        set_bit!(self.0, 59, bit as u64);
    }

    // Contiguous bit 52
    pub fn set_contiguous(&mut self, bit: bool) {
        set_bit!(self.0, 52, bit as u64);
    } 

    // DBM bit 51
    pub fn set_dirty_bit_modifier(&mut self, bit: bool) {
        set_bit!(self.0, 51, bit as u64);
    }

    // nT bit 16
    pub fn set_block_translation_entry(&mut self, bit: bool) {
        set_bit!(self.0, 16, bit as u64);
    }

    // nG bit 11
    pub fn set_not_global(&mut self, bit: bool) {
        set_bit!(self.0, 11, bit as u64);
    }

    // AF bit 10
    pub fn set_access_flag(&mut self, bit: bool) {
        set_bit!(self.0, 10, bit as u64);
    }

    // SH bits [9:8]
    pub fn set_shareability_field(&mut self, setting: Shareability) {
        set_bits!(self.0, 8, setting as u64, 0b11);
    }

    // AP bits [7:6]
    pub fn set_access_permissions(&mut self, ap: AccessPermissions) {
        set_bits!(self.0, 6, ap as u64, 0b11);
    }

    // NS bit 5
    pub fn set_non_secure(&mut self, bit: bool) {
        set_bit!(self.0, 5, bit as u64);
    }
    
    // AttrIndx (MAIR index) bits [4:2]
    pub fn set_memory_attributes_index_field(&mut self, index: AttributeIndex) {
        set_bits!(self.0, 2, index as u64, 0b111);
    }
}

#[derive(Copy, Clone, Debug)]
pub struct TableDescriptorAttributes {
    pub non_secure: bool,
    pub access_permissions: AccessPermissions,
    pub user_exec_never: bool, 
    pub priv_exec_never: bool,
}

#[derive(Debug)]
pub struct TableDescriptor (u64);

impl TableDescriptor {

    pub fn get_output_address(&self, granule_size: GranuleSize) -> PhysicalAddress {
        let (output_start, output_mask) = granule_size.get_table_output_address_start_bit_and_mask();

        let addr: u64 = (self.0 & output_mask) >> output_start;

        PhysicalAddress(addr)
    }

    pub fn is_invalid(&self) -> bool {
        if self.0 & 1 == 1 {
            false
        } else {
            true
        }
    }

    pub fn set_output_address(&mut self, address: PhysicalAddress, granule_size: GranuleSize) {
        //let (output_start, output_mask) = granule_size.get_table_output_address_start_bit_and_mask();
        
        self.0 = (self.0 & !0xFFFF_FFFF_F000) | (address.0 & 0xFFFF_FFFF_F000); // 4kb only
    }

    pub fn set_invalid(&mut self) {
        self.0 = 0;
    }

    pub fn new_level_1_descriptor(non_secure: bool, access_permissions: AccessPermissions, user_execute_never: bool, priviledged_execute_never: bool, granule_size: GranuleSize, address: PhysicalAddress) -> TableDescriptor {
        let mut entry = TableDescriptor(3);

        entry.set_output_address(address, granule_size);
        entry.set_user_execute_never(user_execute_never);
        entry.set_privleged_execute_never(priviledged_execute_never);
        entry.set_access_permissions(access_permissions);
        entry.set_non_secure(non_secure);
        entry
    }

    pub fn new(address: PhysicalAddress, granule_size: GranuleSize, attributes: TableDescriptorAttributes) -> TableDescriptor {
        let mut entry = TableDescriptor(3);

        entry.set_non_secure(attributes.non_secure);
        entry.set_access_permissions(attributes.access_permissions);
        entry.set_user_execute_never(attributes.user_exec_never);
        entry.set_privleged_execute_never(attributes.priv_exec_never);
        entry.set_output_address(address, granule_size);

        entry
    }

    pub fn to_u64(&self) -> u64 {
        self.0
    } 

    // NSTable bit 63
    pub fn set_non_secure(&mut self, bit: bool) {
        set_bit!(self.0, 63, bit as u64);
    }

    // APTable bits [62:61]
    pub fn set_access_permissions(&mut self, ap: AccessPermissions) {
        set_bits!(self.0, 61, ap as u64, 0b11);
    }

    // UXNTable or XNTable bit 60
    pub fn set_user_execute_never(&mut self, bit: bool) {
        set_bit!(self.0, 60, bit as u64);
    }

    // PXNTable bit 59
    pub fn set_privleged_execute_never(&mut self, bit: bool) {
        set_bit!(self.0, 59, bit as u64);
    }

}

pub unsafe fn enable_mmu(level_0_table: &TranslationTable) {
    use cortex_a::regs::{TCR_EL1, ID_AA64MMFR0_EL1, MAIR_EL1, TTBR0_EL1, SCTLR_EL1};
    use cortex_a::barrier;
    use register::cpu::{RegisterReadWrite, RegisterReadOnly};

    let physical_addr_range = ID_AA64MMFR0_EL1.read(ID_AA64MMFR0_EL1::PARange);
    
    MAIR_EL1.write(
        // Attribute 1
        MAIR_EL1::Attr1_Normal_Outer::WriteBack_NonTransient_ReadWriteAlloc +
        MAIR_EL1::Attr1_Normal_Inner::WriteBack_NonTransient_ReadWriteAlloc +

        // Attribute 0 - Device.
        MAIR_EL1::Attr0_Device::nonGathering_nonReordering_EarlyWriteAck

    );

    TTBR0_EL1.set_baddr(level_0_table as *const TranslationTable as u64);

    TCR_EL1.write(
        TCR_EL1::TBI0::Ignored // Top Byte Ignored, whether it should be used with TTBRO_EL1
        + TCR_EL1::IPS.val(physical_addr_range) // Intermediate Physical Address Size
        + TCR_EL1::TG0::KiB_4 // set granule size for TTBRO_EL1
        + TCR_EL1::SH0::Inner // set shareability (inner is typically all processor)
        + TCR_EL1::ORGN0::NonCacheable
        + TCR_EL1::IRGN0::NonCacheable
        + TCR_EL1::EPD0::EnableTTBR0Walks
        + TCR_EL1::EPD1::DisableTTBR1Walks
        + TCR_EL1::T0SZ.val(16)
    );

    barrier::isb(barrier::SY);

    // Enable the MMU and turn on data and instruction caching.
    SCTLR_EL1.modify(SCTLR_EL1::M::Enable + SCTLR_EL1::C::NonCacheable + SCTLR_EL1::I::NonCacheable);

    // Force MMU init to complete before next instruction.
    barrier::isb(barrier::SY);

}

pub unsafe fn disable_mmu() {
    use cortex_a::regs::{SCTLR_EL1};
    use cortex_a::barrier;
    use register::cpu::RegisterReadWrite;

    SCTLR_EL1.modify(SCTLR_EL1::M::Disable);

    // Force MMU init to complete before next instruction.
    barrier::isb(barrier::SY);
}
