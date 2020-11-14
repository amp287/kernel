ENTRY(_start)

EXTERN(__EXCEPTIONS); 

EXTERN(default_interrupt_handler)

EXTERN(interrupt_syn_el0)
EXTERN(interrupt_irq_el0)
EXTERN(interrupt_fiq_el0)
EXTERN(interrupt_serror_el0)

EXTERN(interrupt_syn_elx)
EXTERN(interrupt_irq_elx)
EXTERN(interrupt_fiq_elx)
EXTERN(interrupt_serror_elx)

EXTERN(interrupt_syn_el_lower_64)
EXTERN(interrupt_irq_el_lower_64)
EXTERN(interrupt_fiq_el_lower_64)
EXTERN(interrupt_serror_el_lower_64)

EXTERN(interrupt_syn_el_lower_32)
EXTERN(interrupt_irq_el_lower_32)
EXTERN(interrupt_fiq_el_lower_32)
EXTERN(interrupt_serror_el_lower_32)

PROVIDE(interrupt_syn_el0 = default_interrupt_handler);
PROVIDE(interrupt_irq_el0 = default_interrupt_handler);
PROVIDE(interrupt_fiq_el0 = default_interrupt_handler);
PROVIDE(interrupt_serror_el0 = default_interrupt_handler);

PROVIDE(interrupt_syn_elx = default_interrupt_handler);
PROVIDE(interrupt_irq_elx = default_interrupt_handler);
PROVIDE(interrupt_fiq_elx = default_interrupt_handler);
PROVIDE(interrupt_serror_elx = default_interrupt_handler);

PROVIDE(interrupt_syn_el_lower_64 = default_interrupt_handler);
PROVIDE(interrupt_irq_el_lower_64 = default_interrupt_handler);
PROVIDE(interrupt_fiq_el_lower_64 = default_interrupt_handler);
PROVIDE(interrupt_serror_el_lower_64 = default_interrupt_handler);

PROVIDE(interrupt_syn_el_lower_32 = default_interrupt_handler);
PROVIDE(interrupt_irq_el_lower_32 = default_interrupt_handler);
PROVIDE(interrupt_fiq_el_lower_32 = default_interrupt_handler);
PROVIDE(interrupt_serror_el_lower_32 = default_interrupt_handler);
 
PROVIDE(default_interrupt_handler = interrupt_default_);

SECTIONS
{
    /* Starts at LOADER_ADDR. */
    . = 0x80000;
    /* For AArch64, use . = 0x80000; */
    __start = .;
    __text_start = .;
    .text :
    {
        *(.text.boot) *(.text*)
    }
    . = ALIGN(4096); /* align to page size */
    __text_end = .;
 
    /* static data initialized */
    __rodata_start = .;
    .rodata :
    {
        *(.rodata*)
    }
    . = ALIGN(4096); /* align to page size */
    __rodata_end = .;
 
    __data_start = .;
    .data :
    {
        *(.data*)
    }
    . = ALIGN(4096); /* align to page size */
    __data_end = .;
 
    /* static data uninitialized */
    __bss_start = .;
    
    .bss :
    {
        *(.bss*)
    }
    . = ALIGN(4096); /* align to page size */

    __bss_end = .;
    __bss_size = __bss_end - __bss_start;
    __end = .;
}