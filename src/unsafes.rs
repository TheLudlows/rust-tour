use std::{slice, mem};
use std::alloc::Layout;

#[test]
fn main() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    println!("{:?}", r1);// print address
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    let address = 0x01234usize;
    let r = address as *mut i32;
    // 可能崩溃
    let slice: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };

    static NAME: &str = "four";
    static mut COUNTER: u32 = 0;
    unsafe {
        COUNTER += 100;
        println!("{}", COUNTER)
    }
}

#[test]
fn test_arr_p() {
    let mut arr1 = [1, 2, 3];
    let p = &mut arr1 as *mut [_; 3];
    unsafe {
        // 解引用copy类型
        let mut arr2 = *p;
        arr2[0] = 0;
        println!("{:?}", arr2);
    }
    println!("{:?}", arr1);
}

#[test]
fn test_arr_p1() {
    let mut n = 0;
    let p1 = &mut n as *mut _;
    unsafe {
        *p1 = 1;
        println!("{}", *p1);
    }
    println!("{}", n);
}

#[test]
fn test_p3() {
    let mut s = String::new();

    let ptr: *const String = &s as *const String;

    unsafe {
        let s2 = &*ptr;
        s.push('a');// unsafe 绕过所有权，正常下面打印报错
        println!("{}", s2.len())
    }
}

#[test]
fn test_p4() {
    let mut s = String::new();

    let ptr: *const String = &s as *const String;
    let index: usize = ptr as usize;
    println!("{:?}", ptr);
    println!("{:x}", index);

    let ptr2 = index as *const _;
    unsafe {
        let s2: &String = &*ptr2;
        s.push('a');
        let len = s2.len();
        println!("{:?}", len); // 1
    }
}

fn s_ptr() -> *const String {
    let s = "hello".to_string();
    /*let ptr: *const String =*/ &s as *const String
    //ptr
}

#[test]
fn test_p5() {
    let ptr2: *const String = s_ptr();

    unsafe {
        let s2: &String = &*ptr2;

        let len = s2.len();
        println!("{}", s2); // segmentation fault (core dumped)

        println!("{:?}", len);
    }
}

#[test]
fn test_p6() {
    let mut s = String::new();

    let ptr: *const String = &s as *const String;
    let index: usize = ptr as usize;
    println!("{:?}", ptr);
    println!("{:x}", index);

    let ptr2 = index as * mut _;
    unsafe {
        let s2: &mut String = &mut *ptr2;
        s.push('a');
        println!("{:?}", s2);
    }
}
#[test]
fn test_p7() {
    let array: [i32; 3] = [1, 2, 3];
    println!("{:?}", array); // [1, 2, 3]
    let ptr: *const i32 = &array as *const [i32; 3] as *const i32;
    let index = ptr as usize;
    unsafe {
        let a = (index + 0) as *const i32;
        println!("{:?}", *a); // 1
        let b = (index + 4) as *const i32;
        println!("{:?}", *b); // 2
        let c = (index + 8) as *const i32;
        println!("{:?}", *c); // 3
    }
}

#[test]
fn test_p8() {
    let array: [i32; 3] = [1, 2, 3];
    println!("{:?}", array); // [1, 2, 3]
    let ptr: *const i32 = &array as *const [i32; 3] as *const i32;
    let index = ptr as usize;
    unsafe {
        let a = (index + 0) as *mut i32;
        println!("{:?}", *a); // 1
        let b = (index + 4) as *mut i32;
        *b +=1;
        println!("{:?}", *b); // 2
        let c = (index + 8) as *mut i32;
        println!("{:?}", *c); // 3
    }
}
#[test]
fn test_p9() {
    let array: [i32; 3] = [1, 2, 3];
    println!("{:?}", array); // [1, 2, 3]
    let ptr: *mut i32 = &array as *const [i32; 3] as *mut i32;
    unsafe {
        println!("{:?}", ptr.add(0).read()); // 1
        ptr.add(1).write(456); // 第2个元素
    }
    println!("{:?}", array); // [1, 456, 3]
}

// alloc in heap space
#[test]
fn test_alloc() {
    unsafe {
        // 长度为32的i32数组
        let layout = Layout::from_size_align_unchecked(32 * mem::size_of::<i32>(), mem::size_of::<i32>());
        // 分配内存
        let ptr = std::alloc::alloc(layout) as *mut i32;
        println!("{}", ptr as usize);
        ptr.write(123);
        println!("{:?}", ptr.add(0).read());
        ptr.add(31).write(456);
        println!("{:?}", ptr.add(31).read());
        // 释放内存
        std::alloc::dealloc(ptr as *mut u8, layout);

        let arr = std::ptr::read::<[i32;32]>(ptr as * const [i32;32]);
        println!("{:?}", arr);
    }
}
#[test]
fn test_alloc2() {
    unsafe {
        let layout = Layout::from_size_align_unchecked(32 * mem::size_of::<i32>(), mem::size_of::<i32>());
        let ptr = std::alloc::alloc(layout) as *mut i32;
        let slice = std::slice::from_raw_parts_mut::<i32>(ptr,32);
        slice[0] = 100;
        println!("{:?}", slice);
    }
}
