pub mod bump;

use self::bump::BumpAllocator;

// TODO: Use linker symbol to initialize global allocator
// Linker symbol is imported as `extern static` in rust, but `const` is necessary to use it in global variable.
// Const version of the variable is defined manually as a temporary workaround.
extern "C" {
    static _heap_start: usize;
    static _heap_size: usize;
}

// Change here to move location of the heap
const HEAP_START: usize = 0x81000000;
const HEAP_END: usize = 0x85000000;

// Declear this object to be the allocator
#[global_allocator]
pub static ALLOCATOR: BumpAllocator = BumpAllocator::new(HEAP_START, HEAP_END);
