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

pub struct InterruptInfo {
    pub gpr: [u64; 31],
    pub elr: u64,
    pub spsr: u64,
}

impl fmt::Display for InterruptInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ELR_ELx: {:x}\n", self.elr)?;
        write!(f, "SPSR_ELx: {:x}\n\n", self.spsr)?;
        write!(f, "General Registers:\n")?;
        write!(f, "\t x0:  0x{:x}\t x1:  0x{:x}\n", self.gpr[0], self.gpr[1])?;
        write!(f, "\t x2:  0x{:x}\t x3:  0x{:x}\n", self.gpr[2], self.gpr[3])?; 
        write!(f, "\t x4:  0x{:x}\t x5:  0x{:x}\n", self.gpr[4], self.gpr[5])?; 
        write!(f, "\t x6:  0x{:x}\t x7:  0x{:x}\n", self.gpr[6], self.gpr[7])?;
        write!(f, "\t x8:  0x{:x}\t x9:  0x{:x}\n", self.gpr[8], self.gpr[9])?;
        write!(f, "\t x10: 0x{:x}\t x11: 0x{:x}\n", self.gpr[10], self.gpr[11])?;
        write!(f, "\t x12: 0x{:x}\t x13: 0x{:x}\n", self.gpr[12], self.gpr[13])?;
        write!(f, "\t x14: 0x{:x}\t x15: 0x{:x}\n", self.gpr[14], self.gpr[15])?;
        write!(f, "\t x16: 0x{:x}\t x17: 0x{:x}\n", self.gpr[16], self.gpr[17])?;
        write!(f, "\t x18: 0x{:x}\t x19: 0x{:x}\n", self.gpr[18], self.gpr[19])?;
        write!(f, "\t x20: 0x{:x}\t x21: 0x{:x}\n", self.gpr[20], self.gpr[21])?;
        write!(f, "\t x22: 0x{:x}\t x23: 0x{:x}\n", self.gpr[22], self.gpr[23])?;
        write!(f, "\t x24: 0x{:x}\t x25: 0x{:x}\n", self.gpr[24], self.gpr[25])?;
        write!(f, "\t x26: 0x{:x}\t x27: 0x{:x}\n", self.gpr[26], self.gpr[27])?;
        write!(f, "\t x28: 0x{:x}\t x29: 0x{:x}\n", self.gpr[28], self.gpr[29])?;
        write!(f, "\t x30/lr: 0x{:x}\n", self.gpr[30])
    }
}

#[no_mangle]
pub extern "C" fn interrupt_default_(info: &InterruptInfo, exception_from_lower_level: bool, interrupt_type: InterruptType) {
    use register::cpu::{RegisterReadWrite,RegisterReadOnly};

    let el = exception::get_current_exception_level();
    let esr = regs::ESR_EL1.get();
    let far = regs::FAR_EL1.get();

    serial_println!("----------------- Interrupt -----------------");

    serial_println!("Current EL: {}", el);

    serial_println!("Came from lower EL: {}", exception_from_lower_level);

    serial_println!("Type: {}", interrupt_type);

    serial_println!("Exception class field (ESR_EL1 only): {}", get_bits!(esr, 26, 0x3F));
    serial_println!("Insrtuction specific syndrome (ESR_EL1 only): {}", get_bits!(esr, 0, 0x1FFFFFF));

    serial_println!("FAR_EL1: {:x}", far);

    serial_println!("{}", info);

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

