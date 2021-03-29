
struct Foo{
    a:i32
}

struct Bar{
    a:i32
}

#[test]
fn conv() {
    let f = Foo{a:10};
    // let b = f as Bar; can't work
    let t:&Bar = unsafe { &*(&f as *const Foo as *const Bar) };
}
#[test]
fn  test() {
    let x;
    if true {
        x = Box::new(0);   // x未初始化；仅覆盖值
        println!("{}", x);
    }
    // println!("{}", x); 未初始化
}
use std::mem;
use std::ptr;
use std::mem::MaybeUninit;

#[test]
fn test_uninit() {
    const SIZE: usize = 10;
    let mut x: [Box<u32>; SIZE];
    unsafe {
        // 欺骗Rust说x已经被初始化

        x = MaybeUninit::uninit().assume_init();
        for i in 0..SIZE {
            // 注意：异常安全性不需要考虑；Box不会panic
            ptr::write(&mut x[i], Box::new(i as u32));
        }
    }

    println!("{:?}", x);
    std::sync::sync
}