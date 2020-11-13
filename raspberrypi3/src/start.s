// AArch64 mode
 
// To keep this in the first portion of the binary.
.section ".text.boot"
 
// Make _start global.
.globl _start
 
// Entry point for the kernel. Registers are not defined as in AArch32.
_start:
    // read cpu id, stop slave cores
    mrs     x1, mpidr_el1
    and     x1, x1, #3
    cbz     x1, 2f
    // cpu id > 0, stop
1:  wfe
    b       1b
2:  // cpu id == 0

    // set HCR_EL2.RW to 1, sets el1 to 64-bit 
    mov x1, #0
    orr x1, x1, #(1 << 31)
    msr HCR_EL2, x1
 
    // set stack (for EL1) before our code
    ldr     x1, =_start
    msr     SP_EL1, x1

    // Ensure that floating point register accesses are not trapped
    mov x0, #(0x3 << 20)
    msr CPACR_EL1, x0

    ldr x1, =__interrupt_handlers
    msr VBAR_EL2, x1

    // set M[3:0] to EL1h
    mov x1, #5
    // The exception bit mask bits (DAIF) allow the exception events to be masked. The exception is not taken when the bit is set.
    orr x1, x1, #(1 << 9) // set SPSR_EL2.D where xzr is the zero register
    orr x1, x1, #(1 << 8) // set SPSR_EL2.A
    orr x1, x1, #(1 << 7) // set SPSR_EL2.I
    orr x1, x1, #(1 << 6) // set SPSR_EL.F
 
    msr SPSR_EL2, x1
    
    ldr x1, =_runtime_init
    msr ELR_EL2, x1

    eret

    _runtime_init:
    // clear bss
    ldr     x1, =__bss_start
    ldr     w2, =__bss_size
3:  cbz     w2, 4f
    str     xzr, [x1], #8
    sub     w2, w2, #1
    cbnz    w2, 3b
 
    // jump to C code, should not return
4:  bl      kernel_main
    // for failsafe, halt this core too
    b 5f
5: 
    wfe
    b 5b