.equ SYNCHRONOUS, 0
.equ IRQ, 1
.equ FIQ, 2
.equ SERROR, 3


/// Call the function provided by parameter `\handler` after saving exception context. Provide the
/// context as the first parameter to '\handler'.
.macro CALL_WITH_CONTEXT handler,from_lower_level,type
    sub    sp,  sp,  #16 * 17

    // Store all general purpose registers on the stack.
    stp    x0,  x1,  [sp, #16 * 0]
    stp    x2,  x3,  [sp, #16 * 1]
    stp    x4,  x5,  [sp, #16 * 2]
    stp    x6,  x7,  [sp, #16 * 3]
    stp    x8,  x9,  [sp, #16 * 4]
    stp    x10, x11, [sp, #16 * 5]
    stp    x12, x13, [sp, #16 * 6]
    stp    x14, x15, [sp, #16 * 7]
    stp    x16, x17, [sp, #16 * 8]
    stp    x18, x19, [sp, #16 * 9]
    stp    x20, x21, [sp, #16 * 10]
    stp    x22, x23, [sp, #16 * 11]
    stp    x24, x25, [sp, #16 * 12]
    stp    x26, x27, [sp, #16 * 13]
    stp    x28, x29, [sp, #16 * 14]

    // Add the exception link register (ELR_EL1) and the saved program status (SPSR_EL1).
    mrs    x1,  ELR_EL1
    mrs    x2,  SPSR_EL1

    stp    lr,  x1,  [sp, #16 * 15] // lr is x30
    str    w2,       [sp, #16 * 16] // sp needs to be 16-byte aligned

    // x0 is the first argument for the function called through `\handler`.
    mov    x0,  sp
    mov    x1, \from_lower_level
    mov    x2, \type

    // Call `\handler`.
    bl     \handler
.endm

.global __interrupt_handlers

// The ,"ax",@progbits tells the assembler that the section is allocatable ("a"), executable ("x") and contains data ("@progbits").
.section .exception_vectors, "ax", @progbits

.align 11
__interrupt_handlers:

// current el with sp0
.org 0x000
CALL_WITH_CONTEXT interrupt_syn_el0,0,SYNCHRONOUS
.org 0x080
CALL_WITH_CONTEXT interrupt_irq_el0,0,IRQ
.org 0x100
CALL_WITH_CONTEXT interrupt_fiq_el0,0,FIQ
.org 0x180
CALL_WITH_CONTEXT interrupt_serror_el0,0,SERROR

// current el with spx
.org 0x200
CALL_WITH_CONTEXT interrupt_syn_elx,0,SYNCHRONOUS
.org 0x280
CALL_WITH_CONTEXT interrupt_irq_elx,0,IRQ
.org 0x300
CALL_WITH_CONTEXT interrupt_fiq_elx,0,FIQ
.org 0x380
CALL_WITH_CONTEXT interrupt_serror_elx,0,SERROR

// Lower EL using AArch64
.org 0x400
CALL_WITH_CONTEXT interrupt_syn_el_lower_64,1,SYNCHRONOUS
.org 0x480
CALL_WITH_CONTEXT interrupt_irq_el_lower_64,1,IRQ
.org 0x500
CALL_WITH_CONTEXT interrupt_fiq_el_lower_64,1,FIQ
.org 0x580
CALL_WITH_CONTEXT interrupt_serror_el_lower_64,1,SERROR

// Lower EL using AArch32
.org 0x600
CALL_WITH_CONTEXT interrupt_syn_el_lower_32,1,SYNCHRONOUS
.org 0x680
CALL_WITH_CONTEXT interrupt_irq_el_lower_32,1,IRQ
.org 0x700
CALL_WITH_CONTEXT interrupt_fiq_el_lower_32,1,FIQ
.org 0x780
CALL_WITH_CONTEXT interrupt_serror_el_lower_32,1,SERROR