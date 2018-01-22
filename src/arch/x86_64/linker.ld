ENTRY(start)

SECTIONS {
  . = 1M;

  .boot :
  {
    /* multiboot header at beginning */
    *(.multiboot_header)
  }

  .text :
  {
    *(.text)
  }
}
