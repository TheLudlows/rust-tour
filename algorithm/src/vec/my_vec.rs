use std::{mem, ptr};
use std::alloc::{dealloc, handle_alloc_error, Layout};
use std::alloc::alloc;
use std::alloc::realloc;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};
use std::ptr::Unique;

#[derive(Debug)]
struct MyVec<T> {
    vec: RawVec<T>,
    len: usize,
}

#[derive(Debug)]
struct RawVec<T> {
    ptr: Unique<T>,
    cap: usize,
}

impl<T> RawVec<T> {
    fn new() -> Self {
        Self {
            ptr: Unique::dangling(),
            cap: 0,
        }
    }

    fn grow(&mut self) {
        let align = mem::align_of::<T>();
        let ele_size = mem::size_of::<T>();
        unsafe {
            let layout;
            let (new_cap, ptr) = if self.cap == 0 {
                layout = Layout::from_size_align_unchecked(ele_size, align);
                let ptr = alloc(layout);
                (1, ptr)
            } else {
                let new_cap = self.cap * 2;
                let old_size = self.cap * ele_size;
                assert!(old_size <= isize::MAX as usize / 2, "capacity overflow");

                let new_size = old_size * 2;
                layout = Layout::from_size_align_unchecked(old_size, align);
                let ptr = realloc(self.ptr.as_ptr() as *mut _, layout, new_size);
                (new_cap, ptr)
            };

            if ptr.is_null() {
                handle_alloc_error(layout);
            }
            self.ptr = Unique::new(ptr as *mut _).unwrap();
            self.cap = new_cap;
        }
    }
}

impl<T> Drop for RawVec<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            unsafe {
                let layout = Layout::from_size_align_unchecked(self.cap, mem::align_of::<T>());
                //dealloc(self.ptr.as_ptr() as *mut _, layout);
            }
        }
        println!("drop Raw vec");
    }
}

impl<T> MyVec<T> {
    fn new() -> Self {
        Self {
            vec: RawVec::new(),
            len: 0,
        }
    }

    fn cap(&self) -> usize {
        self.vec.cap
    }

    fn ptr(&self) -> *mut T {
        self.vec.ptr.as_ptr()
    }

    pub fn push(&mut self, t: T) {
        if self.len == self.cap() {
            self.vec.grow();
        }
        unsafe {
            ptr::write(self.ptr().offset(self.len as isize), t);
        }
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            unsafe {
                self.len -= 1;
                Some(ptr::read(self.ptr().offset(self.len as isize)))
            }
        }
    }

    pub fn insert(&mut self, idx: usize, t: T) {
        assert!(idx <= self.len);
        if self.len == self.cap() {
            self.vec.grow();
        }
        unsafe {
            if idx < self.len {
                ptr::copy(self.ptr().offset(idx as isize), self.ptr().offset(idx as isize + 1), self.len - idx)
            }
            ptr::write(self.ptr().offset(idx as isize), t);
            self.len += 1;
        }
    }

    pub fn rm(&mut self, idx: usize) -> T {
        assert!(idx < self.len);
        unsafe {
            self.len -= 1;
            let ret = ptr::read(self.ptr().offset(idx as isize));

            ptr::copy(self.ptr().offset(idx as isize + 1), self.ptr().offset(idx as isize), self.len - idx);
            ret
        }
    }

    fn into_iter(self) -> IntoIter<T> {
        unsafe {
            let row_vec = ptr::read(&self.vec);
            let iter = RawIter::new(&self);
            mem::forget(self);
            IntoIter {
                raw_iter: iter,
                raw_vec: row_vec,
            }
        }
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        if self.len != 0 {
            while let Some(_) = self.pop() {}
        }
        println!("MyVec dropped")
    }
}

impl<T> Deref for MyVec<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe {
            std::slice::from_raw_parts(self.ptr(), self.len)
        }
    }
}

impl<T> DerefMut for MyVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            std::slice::from_raw_parts_mut(self.ptr(), self.len)
        }
    }
}

struct IntoIter<T> {
    raw_vec: RawVec<T>,
    raw_iter: RawIter<T>,
}

struct RawIter<T> {
    start: *const T,
    end: *const T,
}

impl<T> RawIter<T> {
    fn new(slice :  &[T]) -> Self {
        unsafe {
            RawIter {
                start: slice.as_ptr(),
                end: if slice.len() == 0 {
                    slice.as_ptr()
                } else {
                    slice.as_ptr().offset(slice.len() as isize)
                },
            }
        }
    }
}

impl<T> Iterator for RawIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                let ret = Some(ptr::read(self.start));
                self.start = self.start.offset(1);
                ret
            }
        }
    }
}

impl<T> DoubleEndedIterator for RawIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                self.end = self.end.offset(-1);
                Some(ptr::read(self.end))
            }
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.raw_iter.next()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.raw_iter.next_back()
    }
}

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        if self.raw_vec.cap != 0 {
            for _ in &mut *self {}
        }
        println!("drop into iter")
    }
}


#[test]
fn test() {
    let mut v: MyVec<i32> = MyVec::new();
    v.push(1);
    v.push(2);
    assert_eq!(v.pop(), Some(2));
    assert_eq!(v.pop(), Some(1));
    assert_eq!(v.pop(), None);

    v.push(1);
    v.push(2);
    let s = &v[..];
    assert_eq!(s[0], 1);
    //v.insert(0,3);
    println!("{:?}", v);

    let mut v = MyVec::new();
    v.push(1);
    v.push(1);
    v.push(1);
    v.push(1);

    v.insert(1, 2);
    println!("{:?}", v);
    let r = v.rm(1);
    println!("{}", r);
    for i in v.iter() {
        println!("{}", i);
    }

    for i in v.into_iter() {
        println!("{}", i)
    }


    let mut v:MyVec<i32> = MyVec::new();
    //v.push(1);
    //v.push(3);
    //v.push(10);

    let mut it = v.into_iter();
    while let Some(i) = it.next_back() {
        println!("{:?}", i);
    }

    // Phantom data
}