OUTPUT_ARCH("riscv")
ENTRY(_start)

MEMORY
{
    UART (wr)  : ORIGIN = 0x10000000, LENGTH = 0x100
    RAM  (wxa) : ORIGIN = 0x80000000, LENGTH = 1024M
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
    .text : {
        PROVIDE(_text_start = .);
        *(.text.init) *(.text .text.*) 
        PROVIDE(_text_end = .);
    } >RAM AT>RAM :text

    PROVIDE(_global_pointer = .);

    .rodata : {
        PROVIDE(_rodata_start = .);
        *(.rodata .rodata.*)
        PROVIDE(_rodata_end = .);
    } >RAM AT>RAM :rodata
    
    .data : ALIGN(4096) {
        PROVIDE(_data_start = .);
        *(.sdata .sdata.*) *(.data .data.*)
        PROVIDE(_data_end = .);
    } >RAM AT>RAM :data
    PROVIDE(_data_lma = LOADADDR(.data));
    
    .bss : {
        PROVIDE(_bss_start = .);
        *(.sbss .sbss.*) *(.bss .bss.*)
        PROVIDE(_bss_end = .);
    } >RAM AT>RAM :bss
    
    PROVIDE(_ram_start = ORIGIN(RAM));
    PROVIDE(_ram_end = _ram_start + LENGTH(RAM));
    PROVIDE(_heap_start = _bss_end);
    PROVIDE(_stack_start = ALIGN(MIN(_ram_end, _ram_start + 0x08000000), 8));
}
