use cortex_a::{regs, barrier};
use crate::serial_println;
use crate::exception;
use core::fmt;
use crate::get_bits;

#[repr(u64)]
pub enum InterruptType {
    Synchronous = 0,
    Irq = 1,
    Fiq = 2,
    Serror = 3
}

impl fmt::Display for InterruptType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let el: &'static str = match self {
            InterruptType::Synchronous => "Synchronous",
            InterruptType::Irq => "IRQ/vIRQ",
            InterruptType::Fiq => "FIQ/vFIQ",
            InterruptType::Serror => "SError/vSError"
        };
        write!(f, "{}", el)
    }
}

#[no_mangle]
pub extern "C" fn interrupt_default(exception_from_lower_level: bool, interrupt_type: InterruptType) {
    use register::cpu::{RegisterReadWrite,RegisterReadOnly};

    let el = exception::get_current_exception_level();
    
    serial_println!("----------------- Interrupt -----------------");

    serial_println!("Current EL: {}", el);

    serial_println!("Came from lower EL: {}", exception_from_lower_level);

    serial_println!("Type: {}", interrupt_type);

    serial_println!("Return Address (SP_EL0 only): 0x{:X}", regs::ELR_EL1.get());

    let esr = regs::ESR_EL1.get();

    serial_println!("Exception class field (ESR_EL1 only): {}", get_bits!(esr, 26, 0x3F));
    serial_println!("Insrtuction specific syndrome (ESR_EL1 only): {}", get_bits!(esr, 0, 0x1FFFFFF));

    serial_println!("Panicking :(");

    panic!("Interrupt Triggered!")
}

pub unsafe fn interrupt_init() {
    use cortex_a::regs::RegisterReadWrite;
    extern "C" {
        static mut __interrupt_handlers: u64;
    }

    let addr: u64 = &__interrupt_handlers as *const _ as u64;

    regs::VBAR_EL1.set(addr);

    barrier::isb(barrier::SY)
}

