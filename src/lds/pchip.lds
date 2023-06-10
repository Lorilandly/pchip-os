OUTPUT_ARCH("riscv")
ENTRY(_start)

MEMORY
{
  uart_ctrl (rw) : ORIGIN = 0x40600000, LENGTH = 0x10000
  spi_ctrl (rw) : ORIGIN = 0x60400000, LENGTH = 0x10000
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
  .text : {
    . = ALIGN(64);
    PROVIDE(_text_start = .);
    *(.text.init)
    *(.text .text.*)
    PROVIDE(_text_end = .);
    . += 0x40; /* to create a gap between .text and .data b/c ifetch can fetch ahead from .data */
  } >ROM  AT>ROM :text

  PROVIDE(_global_pointer = .);

  .rodata : {
    . = ALIGN(64);
    PROVIDE(_rodata_start = .);
    *(.rodata .rodata.*)
  } >ROM AT>ROM :rodata

  .srodata : {
    . = ALIGN(64);
    *(.srodata.cst16)
    *(.srodata.cst8)
    *(.srodata.cst4)
    *(.srodata.cst2)
    *(.srodata.*)
    PROVIDE(_rodata_end = .);
  } >ROM AT>ROM :rodata

  .data : {
    . = ALIGN(64);
    PROVIDE(_data_start = .);
    *(.data .data.*)
    *(.sdata .sdata.*)
    *(.tohost) /* TODO: Support sections that aren't explicitly listed in this linker script */
    PROVIDE(_data_end = .);
  } >RAM  AT>ROM :data

  PROVIDE(_data_lma = LOADADDR(.data));

  .bss : {
    . = ALIGN(64);
    PROVIDE(_bss_start = .);
    *(.sbss .sbss.*)
    *(.bss .bss.*)
    . = ALIGN(8);
    PROVIDE(_bss_end = .);
  } >RAM AT>RAM :bss

  PROVIDE(_ram_start = ORIGIN(RAM));
  PROVIDE(_stack_start = _bss_end);
  PROVIDE(_stack_end = _stack_start + 0x100000);
  PROVIDE(_ram_end = _ram_start + LENGTH(RAM));

  PROVIDE(_heap_start = _stack_end);
  PROVIDE(_heap_size = _ram_end - _heap_start);
}
