MEMORY {
  BOOT2 (rx)  : ORIGIN = 0x10000000, LENGTH = 0x100
  FLASH (rx)  : ORIGIN = 0x10000100, LENGTH = 2048K - 0x100
  SRAM  (rwx) : ORIGIN = 0x20000000, LENGTH = 256K
  SRAM4 (rwx) : ORIGIN = 0x20040000, LENGTH = 4K
  SRAM5 (rwx) : ORIGIN = 0x20041000, LENGTH = 4K
}

SECTIONS {
  .boot2 ORIGIN(BOOT2) :
  {
    KEEP(*(.boot2));
  } > BOOT2

  .vector_table ORIGIN(FLASH) :
  {
    LONG(ORIGIN(SRAM) + LENGTH(SRAM));
    KEEP(*(.vector_table.reset_vector));
    KEEP(*(.vector_table.exceptions));
    KEEP(*(.vector_table.interrupts));
  } > FLASH

  .text :
  {
    *(.text .text.*);
  } > FLASH

  .rodata :
  {
    *(.rodata .rodata.*);
  } > FLASH

  .data :
  {
    __sdata = .;
    *(.data .data.*);
    __edata = .;
  } > SRAM AT> FLASH

  __sidata = LOADADDR(.data);

  .bss :
  {
    __sbss = .;
    *(.bss .bss.*);
    __ebss = .;
  } > SRAM

  /DISCARD/ :
  {
    *(.ARM.exidx);
    *(.ARM.attributes);
    *(.comment);
  }
}
