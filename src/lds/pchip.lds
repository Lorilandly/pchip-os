OUTPUT_ARCH("riscv")
ENTRY(_start)

Heap_Size = 8*1024;           /* fixed-size, 8kB heap for _sbrk use */
Stack_Size = 2*1024;          /* fixed-size, 2kB main/process stacks */
Thread_Stacks_Size = 32*1024; /* fixed-size, 32kB space for thread main/process stacks */

MEMORY
{
    UART (rw) : ORIGIN = 0x40600000, LENGTH = 0x10000
    SPI (rw) : ORIGIN = 0x60400000, LENGTH = 0x10000
    RAM (rwx) : ORIGIN = 0x80000000, LENGTH = 0x40000000
    ROM (rx) : ORIGIN = 0xC0000000, LENGTH = 0x4000
}

PHDRS
{
    text PT_LOAD;
    rodata PT_LOAD;
    data PT_LOAD;
    bss PT_LOAD;
}

SECTIONS
{
    .text : ALIGN(64) {
        PROVIDE(_text_start = .);
        *(.text.init)
        *(.text .text.*)
        PROVIDE(_text_end = .);
        . += 0x40; /* to create a gap between .text and .data b/c ifetch can fetch ahead from .data */
    } >ROM AT>ROM :text

    PROVIDE(_global_pointer = .);

    .rodata : ALIGN(64) {
        PROVIDE(_rodata_start = .);
        *(.rodata .rodata.*)
    } >ROM AT>ROM :rodata

    .srodata : ALIGN(64) {
        *(.srodata.cst16)
        *(.srodata.cst8)
        *(.srodata.cst4)
        *(.srodata.cst2)
        *(.srodata.*)
        PROVIDE(_rodata_end = .);
    } >ROM AT>ROM :rodata

    .data : ALIGN(64) {
        PROVIDE(_data_start = .);
        *(.data .data.*)
        *(.sdata .sdata.*)
        *(.tohost) /* TODO: Support sections that aren't explicitly listed in this linker script */
        PROVIDE(_data_end = .);
    } >RAM AT>ROM :data

    PROVIDE(_data_lma = LOADADDR(.data));

    .bss : ALIGN(64) {
        PROVIDE(_bss_start = .);
        *(.sbss .sbss.*)
        *(.bss .bss.*)
        . = ALIGN(8);
        PROVIDE(_bss_end = .);
    } >RAM AT>RAM :bss

    /* create boundary labels for 8kB heap, 2kB main stack, 2kB process stack,
     * 32 1kB thread kernel stacks, and 32 1kB thread user stacks */
    .misc : {
        . = ALIGN(8*1024);
        
        /* 8kB heap for alloc */
        _heap_base = .;
        . = . + Heap_Size;
        _heap_limit = .;
        
        /* 2kB main thread stack */
        _u_stack_limit = .;
        . = . + Stack_Size;
        _u_stack_base = .;
    
        /* 2kB kernel stack */
        _m_stack_limit = .;
        . = . + Stack_Size;
        _m_stack_base = .;

        . = ALIGN(32*1024);  

        /* 32kB to hold all thread user stacks */
        _thread_u_stacks_limit = .;
        . = . + Thread_Stacks_Size;
        _thread_u_stacks_base = .;

        /* 32kB to hold all thread kernel stacks */
        _thread_k_stacks_limit = .;
        . = . + Thread_Stacks_Size;
        _thread_k_stacks_base = .;
    } > RAM
}
