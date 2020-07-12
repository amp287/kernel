
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PhysicalAddress (usize);

impl PhysicalAddress {
    pub fn new(addr: usize) -> PhysicalAddress {
        PhysicalAddress(addr)
    }

    pub fn from_reference<T>(item: &T) -> PhysicalAddress {
        let ptr = item as *const _;

        PhysicalAddress(ptr as usize)
    }
}

impl Into<usize> for PhysicalAddress {
    fn into(self) -> usize {
        self.0
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct VirtualAddress (usize);

pub trait VirtualAddressTrait {
    fn new_by_index(level0_index: u64, level1_index: u64, level2_index: u64, level3_index: u64, physical_offset: u16) -> VirtualAddress;
    fn get_level0_index(&self) -> u8;
    fn get_level1_index(&self) -> u8;
    fn get_level2_index(&self) -> u8;
    fn get_level3_index(&self) -> u8;
    fn get_physical_offset(&self) -> u8;
}

pub trait TranslationTableTrait {
    fn new() -> super::super::super::TranslationTable;
    //fn get_entry(&self, index: usize) -> TableEntry;
}

pub trait BlockDescriptorTrait {

}

pub trait TableDescriptorTrait {
    
}

pub enum GranuleSize {
    _4KB, 
    _16KB,
    _64KB,
}

pub trait MMU {
    pub unsafe fn enable_mmu(table: crate::arch::TranslationTable);
    pub unsafe fn diable_mmu();
}