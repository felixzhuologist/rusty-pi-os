use alloc::alloc::{AllocErr, Layout};

use allocator::util::*;
use allocator::linked_list::LinkedList;

// biggest bucket is size ~1MB
const K: usize = 20;

/// A simple allocator that allocates based on size classes.
pub struct Allocator {
    // buckets[i] is a linked list of buckets of size 2^(i+3)
    buckets: [LinkedList; K - 2],
    current: usize,
    end: usize,
}

fn get_bucket(size: usize) -> usize {
    let mut i = 3;
    while (1 << i) < size {
        i += 1;
    } 
    i - 3
}

impl Allocator {
    /// Creates a new bin allocator that will allocate memory from the region
    /// starting at address `start` and ending at address `end`.
    pub fn new(start: usize, end: usize) -> Allocator {
        Allocator {
            buckets: [LinkedList::new(); K - 2],
            current: start,
            end,
        }
    }

    /// Allocates memory. Returns a pointer meeting the size and alignment
    /// properties of `layout.size()` and `layout.align()`.
    ///
    /// If this method returns an `Ok(addr)`, `addr` will be non-null address
    /// pointing to a block of storage suitable for holding an instance of
    /// `layout`. In particular, the block will be at least `layout.size()`
    /// bytes large and will be aligned to `layout.align()`. The returned block
    /// of storage may or may not have its contents initialized or zeroed.
    ///
    /// # Safety
    ///
    /// The _caller_ must ensure that `layout.size() > 0` and that
    /// `layout.align()` is a power of two. Parameters not meeting these
    /// conditions may result in undefined behavior.
    ///
    /// # Errors
    ///
    /// Returning `Err` indicates that either memory is exhausted
    /// (`AllocError::Exhausted`) or `layout` does not meet this allocator's
    /// size or alignment constraints (`AllocError::Unsupported`).
    pub fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
        let bucket = get_bucket(layout.size());
        for node in self.buckets[bucket].iter_mut() {
            let val = usize::from_le(node.value() as usize);
            if val % layout.align() == 0 {
                node.pop();
                return Ok(val as *mut u8);
            }
        }

        let start = align_up(self.current, layout.align());
        let end = start + (1 << (bucket + 3));
        if end >= self.end {
            return Err(AllocErr);
        }
        self.current = end;
        Ok(start as *mut u8)
    }

    /// Deallocates the memory referenced by `ptr`.
    ///
    /// # Safety
    ///
    /// The _caller_ must ensure the following:
    ///
    ///   * `ptr` must denote a block of memory currently allocated via this
    ///     allocator
    ///   * `layout` must properly represent the original layout used in the
    ///     allocation call that returned `ptr`
    ///
    /// Parameters not meeting these conditions may result in undefined
    /// behavior.
    pub fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        unsafe {
            self.buckets[get_bucket(layout.size())].push(ptr as *mut usize);
        }
    }
}
//
// FIXME: Implement `Debug` for `Allocator`.
