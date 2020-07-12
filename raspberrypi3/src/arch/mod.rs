pub mod interface;
mod aarch64;

#[cfg(target_arch = "aarch64")]
pub type TranslationTable = crate::arch::aarch64::memory::TranslationTable;
//type TableEntry = crate::arch::aarch64::memory::