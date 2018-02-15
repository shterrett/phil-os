#![feature(lang_items)]
#![no_std]
#![feature(const_fn)]
#![feature(unique)]
#![feature(ptr_internals)]
#![feature(alloc)]
#![feature(allocator_api)]
#![feature(const_atomic_usize_new)]
#![feature(global_allocator)]
#![feature(abi_x86_interrupt)]

extern crate rlibc;
extern crate volatile;
extern crate spin;
extern crate multiboot2;
#[macro_use]
extern crate bitflags;
extern crate x86_64;
#[macro_use]
extern crate alloc;
#[macro_use]
extern crate once;
extern crate linked_list_allocator;
#[macro_use]
extern crate lazy_static;
extern crate bit_field;

#[macro_use]
mod vga_buffer;
mod memory;
mod interrupts;

use memory::area_frame_allocator::AreaFrameAllocator;
use memory::FrameAllocator;
use memory::BumpAllocator;
use linked_list_allocator::LockedHeap;

pub const HEAP_START: usize = 0o_000_001_000_000_0000;
pub const HEAP_SIZE: usize = 100 * 1024;

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();

#[no_mangle]
pub extern "C" fn rust_main(multiboot_information_address: usize) {
    vga_buffer::clear_screen();
    println!("HelloWorld{}", "!");

    let boot_info = unsafe {
        multiboot2::load(multiboot_information_address)
    };

    enable_nxe_bit();
    enable_write_protect_bit();

    let mut memory_controller = memory::init(boot_info);

    unsafe {
        HEAP_ALLOCATOR.lock().init(HEAP_START, HEAP_START + HEAP_SIZE);
    };

    use alloc::boxed::Box;
    let heap_test = Box::new(42);

    for i in 0..10000 {
        format!("Some String");
    }

    interrupts::init(&mut memory_controller);
    x86_64::instructions::interrupts::int3();

    stack_overflow();

    unsafe { // cause page fault
        *(0xdeadbeaf as *mut u64) = 42;
    }


    println!("It did not crash!");

    loop {};
}

fn stack_overflow() -> u64 {
    u64::max_value() + stack_overflow()
}

#[lang = "eh_personality"] #[no_mangle] pub extern fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str, line: u32) -> ! {
    println!("\n\nPANIC in {} at line {}:", file, line);
    println!("    {}", fmt);
    loop{}
}

fn enable_nxe_bit() {
    use x86_64::registers::msr::{ IA32_EFER, rdmsr, wrmsr };

    let nxe_bit = 1 << 11;
    unsafe {
        let efer = rdmsr(IA32_EFER);
        wrmsr(IA32_EFER, efer | nxe_bit);
    }
}

fn enable_write_protect_bit() {
    use x86_64::registers::control_regs::{ cr0, cr0_write, Cr0 };

    unsafe { cr0_write(cr0() | Cr0::WRITE_PROTECT) };
}
