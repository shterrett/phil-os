ENTRY(start)

SECTIONS {
  . = 1M;

  .boot :
  {
    /* multiboot header at beginning */
    KEEP(*(.multiboot_header))
  }

  .text :
  {
    *(.text)
  }
}
