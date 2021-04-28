use std::alloc::{self, Layout};
use std::mem;
use std::ops;
use std::slice;

#[derive(Debug)]
pub struct MyArray<T: Sized> {
    ptr: *mut T,
    capacity: usize,
    len: usize,
}

impl<T: Sized> MyArray<T> {
    pub fn with_capacity(capacity: usize) -> MyArray<T> {
        let elem_size = mem::size_of::<T>();
        let alloc_size = capacity * elem_size;
        let align = mem::align_of::<T>();

        let layout = Layout::from_size_align(alloc_size, align).unwrap();

        let ptr = unsafe {
            alloc::alloc(layout) as *mut T
        };

        MyArray {
            ptr,
            capacity,
            len: 0,
        }
    }

    pub fn double(&mut self) {
        let elem_size = mem::size_of::<T>();
        let new_cap = 2 * self.capacity;
        let new_size = new_cap * elem_size;

        let align = mem::align_of::<T>();
        let size = mem::size_of::<T>() * self.capacity;
        let layout = Layout::from_size_align(size, align).unwrap();

        unsafe {
            self.ptr = alloc::realloc(self.ptr as *mut u8, layout, new_size) as *mut T;
        }

        self.capacity = new_cap;
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, value: T) {
        if self.len == self.capacity {
            self.double()
        }

        unsafe {
            self.ptr.add(self.len).write(value);
            self.len += 1;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe {
                Some(self.ptr.add(self.len).read())
            }
        }
    }

    pub fn as_slice(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.ptr, self.len) }
    }

    pub fn as_mut_slice(&self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}

impl<T: Sized> Drop for MyArray<T> {
    fn drop(&mut self) {
        let align = mem::align_of::<T>();
        let size = mem::size_of::<T>() * self.capacity;
        let layout = Layout::from_size_align(size, align).unwrap();

        for _ in 0..self.len {
            self.pop();
        }

        unsafe {
            alloc::dealloc(self.ptr as *mut u8, layout);
        }
    }
}

impl<T> ops::Deref for MyArray<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> ops::DerefMut for MyArray<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

#[test]
fn main() {
    let mut array: MyArray<i32> = MyArray::with_capacity(3);
    array.push(1);
    array.push(2);
    array.push(3);

    println!("{:?}", array[0]); // 1
    println!("{:?}", &array[..]); // [1, 2, 3]
    array.pop();
    println!("{:?}", &array[..]); // [1, 2]
    array.push(4);
    array.push(5);

    println!("{:?}", &array[..]); // [1, 2, 4, 5]
    println!("{:?}", array.capacity()); // 6
}

#[test]
fn test_ptr() {
    let mut array = MyArray::with_capacity(3);

    array.push(1);
    array.push(2);
    array.push(3);

    println!("{:?}", &array[..]); // [1, 2, 3]

    let ptr = &mut array as *mut MyArray<i32>;

    let mut array2: MyArray<i32> = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
    let ptr2 = &mut array2 as *mut MyArray<i32>;
    println!("{:?}", ptr);
    unsafe {
        std::ptr::copy(ptr, ptr2, 1);
    }
    println!("{:?}", array2);
    println!("{:?}", array)
    /*
    assert!(array.ptr == array2.ptr);

     println!("{:?}", &array[..]); // [1, 2, 3]
     println!("{:?}", &array2[..]); // [1, 2, 3]

     array[1] = 123;

     println!("{:?}", &array[..]); // [1, 123, 3]
     println!("{:?}", &array2[..]); // [1, 123, 3]

     drop(array2);

     println!("{:?}", &array[..]); // [0, 0, 3]
     */
}