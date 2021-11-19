use std::{fmt, ptr};
use std::fmt::{Formatter, Pointer};

#[test]
fn test_ptr() {
    #[derive(Debug)]
    struct Foo {
        a: i32,
    }

    impl Pointer for Foo {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            let ptr = self as *const Self;
            ptr.fmt(f)
        }
    }

    let f1 = Foo { a: 10 };
    unsafe {
        let f2 = ptr::read(&f1);
        println!("{:p}", f1);
        println!("{:p}", f2);
    }
}

trait P {
    fn fmtt(&self);
}

impl<T> P for T {
    fn fmtt(&self) {
        println!("fuck");
    }
}

#[test]
fn test_trait() {
    P::fmtt(&1);
}