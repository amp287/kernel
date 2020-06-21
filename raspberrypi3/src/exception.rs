use cortex_a::{regs, asm};
use core::fmt;
use register::cpu::RegisterReadOnly;
use crate::set_bits;

#[allow(non_camel_case_types)]
pub enum ExceptionLevel {
    EL_0,
    EL_1,
    EL_2,
    EL_3,
}

impl fmt::Display for ExceptionLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let el: &'static str = match self {
            ExceptionLevel::EL_0 => "EL0",
            ExceptionLevel::EL_1 => "EL1",
            ExceptionLevel::EL_2 => "EL2",
            ExceptionLevel::EL_3 => "EL3"
        };
        write!(f, "{}", el)
    }
}

pub fn get_current_exception_level() -> ExceptionLevel {
    match regs::CurrentEL.read_as_enum(regs::CurrentEL::EL) {
        Some(regs::CurrentEL::EL::Value::EL2) => ExceptionLevel::EL_2,
        Some(regs::CurrentEL::EL::Value::EL1) => ExceptionLevel::EL_1,
        Some(regs::CurrentEL::EL::Value::EL0) => ExceptionLevel::EL_0,
        _ => ExceptionLevel::EL_3,
    }
}

pub unsafe fn set_sp_el0(stack_pointer: u64) {
    asm!("msr SP_EL0, {}",
        in(reg) stack_pointer
    );
}

pub unsafe fn set_exception_return_el_1(function: fn()) {
    asm!("msr ELR_EL1, {}",
        in(reg) function as *const u64
    );
}

pub unsafe fn el1_to_el0(stack_pointer: u64, function: fn()) {
    use register::cpu::RegisterReadWrite;
    let mut spsr_el1: u32 = 0;

    set_bits!(spsr_el1, 0, 0, 0b111); // go to el0
    set_bits!(spsr_el1, 6, 0b1111, 0b1111); // D,A,I,F interrupt masks

    regs::SPSR_EL1.set(spsr_el1);

    set_sp_el0(stack_pointer);

    set_exception_return_el_1(function);

    asm::eret();
}