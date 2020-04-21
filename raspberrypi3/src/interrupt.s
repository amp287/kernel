.equ SYNCHRONOUS, 0
.equ IRQ, 1
.equ FIQ, 2
.equ SERROR, 3


/// Call the function provided by parameter `\handler` after saving exception context. Provide the
/// context as the first parameter to '\handler'.
.macro CALL_WITH_CONTEXT handler,from_lower_level,type

    stp x0, x1, [sp, #-16]!
    stp x2, x3, [sp, #-16]!
    stp x4, x5, [sp, #-16]!
    stp x6, x7, [sp, #-16]!
    stp x8, x9, [sp, #-16]!
    stp x10, x11, [sp, #-16]!
    stp x12, x13, [sp, #-16]!
    stp x14, x15, [sp, #-16]!
    stp x16, x17, [sp, #-16]!
    stp x18, x19, [sp, #-16]!
    stp x20, x21, [sp, #-16]!
    stp x22, x23, [sp, #-16]!
    stp x24, x25, [sp, #-16]!
    stp x26, x27, [sp, #-16]!
    stp x28, x29, [sp, #-16]!
    str x30, [sp, #-16]!

    mov x0, \from_lower_level
    mov x1, \type

    bl \handler
.endm

.globl __interrupt_handlers

// The ,"ax",@progbits tells the assembler that the section is allocatable ("a"), executable ("x") and contains data ("@progbits").
.section .exception_vectors, "ax", @progbits

.align 11
__interrupt_handlers:

// current el with sp0
.org 0x000
CALL_WITH_CONTEXT interrupt_default,0,SYNCHRONOUS
.org 0x080
CALL_WITH_CONTEXT interrupt_default,0,IRQ
.org 0x100
CALL_WITH_CONTEXT interrupt_default,0,FIQ
.org 0x180
CALL_WITH_CONTEXT interrupt_default,0,SERROR

// current el with spx
.org 0x200
CALL_WITH_CONTEXT interrupt_default,0,SYNCHRONOUS
.org 0x280
CALL_WITH_CONTEXT interrupt_default,0,IRQ
.org 0x300
CALL_WITH_CONTEXT interrupt_default,0,FIQ
.org 0x380
CALL_WITH_CONTEXT interrupt_default,0,SERROR

// Lower EL using AArch64
.org 0x400
CALL_WITH_CONTEXT interrupt_default,1,SYNCHRONOUS
.org 0x480
CALL_WITH_CONTEXT interrupt_default,1,IRQ
.org 0x500
CALL_WITH_CONTEXT interrupt_default,1,FIQ
.org 0x580
CALL_WITH_CONTEXT interrupt_default,1,SERROR

// Lower EL using AArch32
.org 0x600
CALL_WITH_CONTEXT interrupt_default,1,SYNCHRONOUS
.org 0x680
CALL_WITH_CONTEXT interrupt_default,1,IRQ
.org 0x700
CALL_WITH_CONTEXT interrupt_default,1,FIQ
.org 0x780
CALL_WITH_CONTEXT interrupt_default,1,SERROR