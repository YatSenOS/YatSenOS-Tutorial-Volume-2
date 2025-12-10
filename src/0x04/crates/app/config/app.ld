ENTRY(_start)

SECTIONS {
  . = 0x111100000000;

  .rodata ALIGN(4K):
  {
    *(.rodata .rodata.*)
  }

  .text ALIGN(4K):
  {
    *(.text .text.*)
  }

  .data ALIGN(4K):
  {
    *(.data .data.*)
  }

  .got ALIGN(4K):
  {
    *(.got .got.*)
  }

  .bss ALIGN(4K):
  {
    *(.bss .bss.*)
  }
}
