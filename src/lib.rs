#![feature(lang_items)]
#![no_std]
#![feature(const_fn)]
#![feature(unique)]
#![feature(ptr_internals)]

extern crate rlibc;
extern crate volatile;
extern crate spin;

#[macro_use]
mod vga_buffer;

use core::fmt::Write;

#[no_mangle]
pub extern fn rust_main() {
    vga_buffer::clear_screen();
    println!("Hello World{}", "!");
    println!("{}", { println!("inner"); "outer" });
    loop {};
}

#[lang = "eh_personality"] #[no_mangle] pub extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}
