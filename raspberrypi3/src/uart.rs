use core::ptr;
use volatile_register::{RW, RO};

pub fn delay(cycles: u32) {
    for _ in 0..cycles  {
       unsafe{ asm!("nop");}
    }
}

#[allow(non_snake_case)]
#[repr(C, packed)]
struct Uart {
    DR: RW<u32>,        // Data Register
    RSRECR: RW<u32>,    // IDK
    FR: RW<u32>,        // Flag register
    ILPR: RW<u32>,      // not in use
    IBRD: RW<u32>,      // Integer Baud Rate Divisor
    FBRD: RW<u32>,      // Fractional Baud Rate Divisor
    LCRH: RW<u32>,      // Line control register
    CR: RW<u32>,        // Control register
    IFLS: RW<u32>,      // Interupt FIFO Level Select Register
    IMSC: RW<u32>,      // Interupt Mask Set Clear Register
    RIS: RO<u32>,       // Raw Interupt Status Register
    MIS: RW<u32>,       // Masked Interupt Status Register
    ICR: RW<u32>,       // Interupt Clear Register
    DMACR: RW<u32>,     // DMA Control Register
    ITCR: RW<u32>,      // Test Control Register
    ITIP: RW<u32>,      // Integration Test Input Register
    ITOP: RW<u32>,      // Integration Test Output Register
}

// TODO: Fix this
pub unsafe fn uart_init() {
    let gpp_ud = 0x009 as *mut u32;
    let gppud_clk = 0x008 as *mut u32; 
    let uart = 0x3F201000 as *const Uart;

    (*uart).CR.write(0);

    ptr::write_volatile(gpp_ud, 0x0);
    delay(150);

    ptr::write_volatile(gppud_clk, (1 << 14) | (1 << 15));
    delay(150);

    ptr::write_volatile(gppud_clk, 0);

    (*uart).ICR.write(0x7FF);

    (*uart).IBRD.write(1);
    (*uart).FBRD.write(40);
    (*uart).LCRH.write((1 << 4) | (1 << 5) | (1 << 6));
    (*uart).IMSC.write((1 << 1) | (1 << 4) | (1 << 5) | (1 << 6) |
    (1 << 7) | (1 << 8) | (1 << 9) | (1 << 10));
    (*uart).CR.write((1 << 0) | (1 << 8) | (1 << 9));

}

pub unsafe fn uart_put(character: u8) {
    let uart = 0x3F20_1000 as *const Uart;

    while (*uart).FR.read() & (1 << 5) != 0 {
        
     }
	(*uart).DR.write(character.into());
}