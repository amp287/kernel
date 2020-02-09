/// Call the function provided by parameter `\handler` after saving exception context. Provide the
/// context as the first parameter to '\handler'.
.macro CALL_WITH_CONTEXT handler
    // Make room on the stack for the exception context.
    sub    sp,  sp,  #16 * 17

    // Store all general purpose registers on the stack.
    push {x0}
    push {x1}
    push {x2}
    push {x3}
    push {x4}
    push {x5}
    push {x6}
    push {x7}
    push {x8}
    push {x9}
    push {x10}
    push {x11}
    push {x12}
    push {x13}
    push {x14}
    push {x15}
    push {x16}
    push {x17}
    push {x18}
    push {x19}
    push {x20}
    push {x21}
    push {x22}
    push {x23}
    push {x24}
    push {x25}
    push {x26}
    push {x27}
    push {x28}
    push {x29}
    push {x30}

    // Add the exception link register (ELR_EL1) and the saved program status (SPSR_EL1).
    mrs    x1,  ELR_EL1
    mrs    x2,  SPSR_EL1

    stp    lr,  x1,  [sp, #16 * 15]
    str    w2,       [sp, #16 * 16]
 
    // x0 is the first argument for the function called through `\handler`.
    mov    x0,  sp

    // Call `\handler`.
    bl     \handler

    // After returning from exception handling code, replay the saved context and return via `eret`.
    b      __exception_restore_context
.endm