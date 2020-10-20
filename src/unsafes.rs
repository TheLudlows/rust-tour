use std::slice;

fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

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
