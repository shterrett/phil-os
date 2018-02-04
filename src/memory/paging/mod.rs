use memory::PAGE_SIZE;
mod table;
mod entry;

const ENTRY_COUNT: usize = 512;

pub type PhysicalAddress = usize;
pub type VirtualAddress = usize;

pub struct Page {
    number: usize
}
