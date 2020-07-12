#[derive(Clone, Copy)]
#[repr(C)]
#[repr(align(65536))]
pub struct TranslationTable ([u64; 512]);

pub const TABLE_INIT: TranslationTable =  TranslationTable([0; 512]);

