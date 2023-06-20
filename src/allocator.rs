pub mod bump;

use self::bump::BumpAllocator;

// TODO: Use linker symbol to initialize global allocator
// Linker symbol is imported as extern C static in rust, while const is necessary for this purpose.
// Const is defined manually as a temporary solution.
extern "C" {
    static _heap_start: usize;
    static _heap_size: usize;
}

const HEAP_START: usize = 0x82000000;
const HEAP_END: usize = 0x86ffffff;

#[global_allocator]
pub static ALLOCATOR: BumpAllocator = BumpAllocator::new(HEAP_START, HEAP_END);
