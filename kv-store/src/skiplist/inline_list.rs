use bytes::Bytes;
use rand::random;
use std::cmp::Ordering as CmpOrdering;
use std::mem;
use std::ptr;
use std::ptr::{null_mut, NonNull};
use std::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};
use std::sync::Arc;

use crate::skiplist::Allocator;

use super::Arena;
use super::HEIGHT_INCREASE;
use super::KeyComparator;
use super::MAX_HEIGHT;


#[derive(Debug)]
#[repr(C)]
pub struct Node {
    key: Bytes,
    value:Bytes,
    height: usize,
    // The actual size will vary depending on the height that a node
    // was allocated with.
    next_nodes: [AtomicPtr<Self>; MAX_HEIGHT],
}

impl Node {
    fn new(key: Bytes, value:Bytes, height: usize, arena: &Arena) -> *mut Self {
        let size =
            mem::size_of::<Self>() - (MAX_HEIGHT - height) * mem::size_of::<AtomicPtr<Self>>();
        let align = mem::align_of::<Self>();
        let off = arena.alloc(align, size);
        assert_ne!(off, 0);
        unsafe {
            let node: *mut Node  = arena.get_mut(off);
            let node = &mut *node;
            ptr::write(&mut node.key, key);
            ptr::write(&mut node.value, value);
            ptr::write(&mut node.height, height);
            ptr::write_bytes(node.next_nodes.as_mut_ptr(), 0, height);
            node
        }
    }

    #[inline]
    // height, 0-based index
    fn get_next(&self, height: usize) -> *mut Node {
        self.next_nodes[height].load(Ordering::SeqCst)
    }

    #[inline]
    // height, 0-based index
    fn set_next(&self, height: usize, node: *mut Node) {
        self.next_nodes[height].store(node, Ordering::SeqCst);
    }

    #[inline]
    fn key(&self) -> &[u8] {
        &self.key
    }
}

struct InlineSkipListInner {
    // Current height. 1 <= height <= kMaxHeight. CAS.
    height: AtomicUsize,
    head: NonNull<Node>,
    arena: Arena,
    // The total size memory skiplist served
    //
    // Note:
    // We only alloc space for `Node` in arena without the content of `key` (only `Bytes` which is pretty small).
    size: AtomicUsize,
}

unsafe impl Send for InlineSkipListInner {}
unsafe impl Sync for InlineSkipListInner {}

impl Drop for InlineSkipListInner {
    fn drop(&mut self) {
        let mut node = self.head.as_ptr();
        loop {
            let next = unsafe { (&*node).get_next(0) };
            if !next.is_null() {
                unsafe {
                    ptr::drop_in_place(node);
                }
                node = next;
                continue;
            }
            unsafe { ptr::drop_in_place(node) };
            return;
        }
    }
}

#[derive(Clone)]
pub struct InlineSkipList<C: KeyComparator> {
    inner: Arc<InlineSkipListInner>,
    comparator: C,
}

impl<C> InlineSkipList<C>
where
    C: KeyComparator,
{
    pub fn with_capacity(comparator: C, cap:usize) -> Self {
        let arena = Arena::with_capacity(cap);
        let head = Node::new(Bytes::new(), Bytes::new(), MAX_HEIGHT, &arena);
        Self {
            inner: Arc::new(InlineSkipListInner {
                height: AtomicUsize::new(1),
                head: unsafe { NonNull::new_unchecked(head) },
                arena,
                size: AtomicUsize::new(0),
            }),
            comparator,
        }
    }

    // findNear finds the node near to key.
    // If less=true, it finds rightmost node such that node.key < key (if allowEqual=false) or
    // node.key <= key (if allowEqual=true).
    // If less=false, it finds leftmost node such that node.key > key (if allowEqual=false) or
    // node.key >= key (if allowEqual=true).
    // Returns the node found. The bool returned is true if the node has key equal to given key.
    fn find_near(&self, key: &[u8], less: bool, allow_equal: bool) -> *const Node {
        let head = self.inner.head.as_ptr();
        let mut x = head;
        let mut level = self.get_height() - 1;
        loop {
            unsafe {
                // Assume x.key < key
                let next_ptr = (*x).get_next(level);
                if next_ptr.is_null() {
                    // x.key < key < END OF LIST
                    if level > 0 {
                        level -= 1;
                        continue;
                    }
                    // height = 0 or it's a head node
                    if !less || x == head {
                        return ptr::null();
                    }
                    return x;
                }
                let next = &*next_ptr;
                match self.comparator.compare_key(key, &next.key) {
                    CmpOrdering::Greater => {
                        // x.key < next.key < key. We can continue to move right.
                        x = next_ptr;
                        continue;
                    }
                    CmpOrdering::Equal => {
                        // x.key < key == next.key.
                        if allow_equal {
                            return next_ptr;
                        }
                        if !less {
                            // We want >, so go to base level to grab the next bigger note.
                            return next.get_next(0);
                        }
                        // We want <. If not base level, we should go closer in the next level.
                        if level > 0 {
                            level -= 1;
                            continue;
                        }
                        // On base level. Return x.
                        // Try to return x. Make sure it is not a head node.
                        if x == head {
                            return ptr::null();
                        }
                        return x;
                    }
                    CmpOrdering::Less => {
                        // x.key < key < next.key.
                        if level > 0 {
                            level -= 1;
                            continue;
                        }
                        // At base level. Need to return something.
                        if !less {
                            return next_ptr;
                        }
                        if x == head {
                            return ptr::null();
                        }
                        return x
                    }
                }
            }
        }
    }

    pub fn put(&self, key: impl Into<Bytes>, value: impl Into<Bytes>) {
        let key: Bytes = key.into();
        self.inner.size.fetch_add(key.len(), Ordering::SeqCst);
        let mut list_height = self.get_height();
        let mut prev = vec![null_mut(); MAX_HEIGHT + 1];
        let mut next = vec![null_mut(); MAX_HEIGHT + 1];
        prev[list_height] = self.inner.head.as_ptr();
        for i in (0..list_height).rev() {
            // Use higher level to speed up for current level.
            let (p, n) = self.find_splice_for_level(&key, prev[i + 1], i);
            prev[i] = p;
            next[i] = n;
            assert_ne!(prev[i], next[i]);
        }
        let height = random_height();
        let np = Node::new(key, value.into(), height, &self.inner.arena);

        while height > list_height {
            match self.inner.height.compare_exchange_weak(
                list_height,
                height,
                Ordering::SeqCst,
                Ordering::SeqCst,
            ) {
                Ok(_) => break,
                Err(h) => list_height = h,
            }
        }
        let node = unsafe { &*np };

        // We always insert from the base level and up. After you add a node in base level, we cannot
        // create a node in the level above because it would have discovered the node in the base level.
        for i in 0..height {
            loop {
                if prev[i].is_null() {
                    assert!(i > 1);
                    // We haven't computed prev, next for this level because height exceeds old listHeight.
                    // For these levels, we expect the lists to be sparse, so we can just search from head.
                    let (p, n) = self.find_splice_for_level(&node.key, self.inner.head.as_ptr(), i);

                    // Someone adds the exact same key before we are able to do so. This can only happen on
                    // the base level. But we know we are not on the base level.
                    prev[i] = p;
                    next[i] = n;
                    assert_ne!(p, n);
                }
                unsafe {
                    node.set_next(i, next[i]);
                    match &(*prev[i]).next_nodes[i].compare_exchange(
                        next[i],
                        np,
                        Ordering::SeqCst,
                        Ordering::SeqCst,
                    ) {
                        Ok(_) => {
                            break;
                        }
                        Err(_) => {
                            // CAS failed. We need to recompute prev and next.
                            // It is unlikely to be helpful to try to use a different level as we redo the search,
                            // because it is unlikely that lots of nodes are inserted between prev[i] and next[i].
                            let (p, n) = self.find_splice_for_level(&node.key, prev[i], i);
                            if p == n {
                                // In wickdb, this should never happen
                                assert_eq!(i, 0, "Equality can happen only on base level");
                                ptr::drop_in_place(np);
                                return;
                            }
                            prev[i] = p;
                            next[i] = n;
                        }
                    }
                }
            }
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.find_last().is_null()
    }

    pub fn len(&self) -> usize {
        let mut node = self.inner.head.as_ptr();
        let mut count = 0;
        loop {
            let next = unsafe { (&*node).get_next(0) };
            if !next.is_null() {
                count += 1;
                node = next;
                continue;
            }
            return count;
        }
    }

    #[inline]
    pub fn total_size(&self) -> usize {
        self.inner.size.load(Ordering::SeqCst) + self.inner.arena.len() as usize
    }

    fn find_last(&self) -> *mut Node {
        let mut x = self.inner.head.as_ptr();
        let mut height = self.get_height() - 1;
        loop {
            unsafe {
                let next = (*x).get_next(height);
                if next.is_null() {
                    if height == 0 {
                        if x == self.inner.head.as_ptr() {
                            return null_mut();
                        }
                        return x;
                    } else {
                        height -= 1;
                    }
                } else {
                    x = next;
                }
            }
        }
    }

    fn find_splice_for_level(
        &self,
        key: &[u8],
        mut before: *mut Node,
        height: usize,
    ) -> (*mut Node, *mut Node) {
        loop {
            unsafe {
                let next = (&*before).get_next(height);
                if next.is_null() {
                    return (before, null_mut());
                } else {
                    match self.comparator.compare_key(key, &(*next).key) {
                        CmpOrdering::Equal => return (next, next),
                        CmpOrdering::Less => return (before, next),
                        CmpOrdering::Greater => {
                            before = next;
                        }
                    }
                }
            }
        }
    }

    fn get_height(&self) -> usize {
        self.inner.height.load(Ordering::Relaxed)
    }

    pub fn get(&self, key: &[u8]) -> Option<&Bytes> {
        let node = self.find_near(key, false, true) ;
        if node.is_null() {
            return None;
        }
        if self.comparator.same_key(&unsafe { &*node }.key, key) {
            return unsafe { Some(&(*node).value) };
        }
        None
    }
}

fn random_height() -> usize {
    let mut height = 1;
    while height < MAX_HEIGHT && random::<u32>() < HEIGHT_INCREASE {
        height += 1;
    }
    height
}