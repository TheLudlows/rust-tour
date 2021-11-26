use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use super::Allocator;
use std::mem;
struct InnerArena {
    len: AtomicUsize,
    cap: usize,
    pub ptr: *mut u8,
}

struct Arena {
    inner: Arc<InnerArena>,
}

impl Arena {
    pub fn wiht_cap(cap: usize) -> Self {
        let mut v: Vec<u64> = Vec::with_capacity(cap >> 3);
        let ptr =  v.as_mut_ptr() as *mut u8;
        mem::forget(v);
        Self {
            inner: Arc::new(InnerArena {
                len: AtomicUsize::new(1),
                cap: cap >> 3 << 3,
                ptr,
            }),
        }
    }
}
impl Allocator for Arena {
    fn len(&self) -> usize {
        self.inner.len.load(Ordering::Acquire)
    }

    fn capacity(&self) -> usize {
        self.inner.cap
    }

    fn allocate(&self, align: usize, size: usize) -> usize {
        let align_mask = align - 1;
        // Leave enough padding for align.
        let size = size + align_mask;
        let offset = self.inner.len.fetch_add(size, Ordering::SeqCst);
        // (offset + align_mask) / align * align.
        let ptr_offset = (offset + align_mask) & !align_mask;

        ptr_offset
    }
}

#[cfg(test)]
mod test {
    use crate::skiplist::Allocator;

    #[test]
    fn test() {
        let arena = super::Arena::wiht_cap(1024);
        let align = 256;
        let size = 8;
        println!("{}", arena.inner.ptr as usize);
        let r = arena.allocate(align, size);
        println!("{}, {}", r, arena.len());
        let r = arena.allocate(align, size);
        println!("{}", arena.len());
        let r = arena.allocate(align, size);
        println!("{}", arena.len());
        println!("{}", std::mem::align_of::<Vec<u64>>())
    }
}
