section .multiboot_header
header_start:
  dd 0xe85250d6 ; magic number - multiboot 2
  dd 0 ; protected i386 - architecture 0
  dd header_end - header_start ; header length
  ; checksum
  dd 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))

  ; optional multiboot tags

  ; required end tag
  dw 0 ; type
  dw 0 ; flags
  dw 8 ; size
header_end:
