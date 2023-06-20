use core::ptr::null_mut;
use core::sync::atomic::{AtomicUsize, Ordering};

use alloc::alloc::{GlobalAlloc, Layout};

pub struct BumpAllocator {
    heap_start: usize,
    heap_end: usize,
    next: AtomicUsize,
    allocations: AtomicUsize,
}

impl BumpAllocator {
    /// Creates a new empty bump allocator.
    pub const fn new(heap_start: usize, heap_end: usize) -> Self {
        BumpAllocator {
            heap_start,
            heap_end,
            next: AtomicUsize::new(heap_start),
            allocations: AtomicUsize::new(0),
        }
    }
}

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();

        // `Layout` contract forbids making a `Layout` with align=0, or align not power of 2.
        // So we can safely use a mask to ensure alignment without worrying about UB.
        let align_mask = !(align - 1);

        let mut inext = 0;
        if self
            .next
            .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |mut next| {
                // Align up
                inext = (next + align - 1) & align_mask;
                if size + inext > self.heap_end {
                    return None;
                }
                next = inext + size;
                inext = next;
                Some(next)
            })
            .is_err()
        {
            return null_mut();
        };
        self.allocations.fetch_add(1, Ordering::SeqCst);
        (inext) as *mut u8
    }

    /// Decrement the number of allocations.
    /// When the number of allocations reaches 0, free the entire heap
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        if self.allocations.fetch_sub(1, Ordering::AcqRel) == 1 {
            self.next.store(self.heap_start, Ordering::Release);
        }
    }
}
