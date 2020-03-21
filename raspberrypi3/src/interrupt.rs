use cortex_a::{regs, barrier};

#[no_mangle]
pub extern "C" fn interrupt_default() {
    panic!("Interrupt Triggered!")
}

pub unsafe fn interrupt_init() {
    use cortex_a::regs::RegisterReadWrite;
    extern "C" {
        static mut __interrupt_handlers: u64;
    }

    let addr: u64 = &__interrupt_handlers as *const _ as u64;

    regs::VBAR_EL1.set(__interrupt_handlers);

    barrier::isb(barrier::SY)
}

pub fn generate_interrupt() {
    unsafe {
        asm!("HVC #0");
    }
    
}

