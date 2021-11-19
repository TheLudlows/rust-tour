use std::ops::Mul;

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

#[test]
fn func_pointer() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);
}

#[test]
fn trans_p() {
    let f = Foo { i: 32 };
    print(&f);
    println!("{:p}", &f)
}

#[derive(Debug)]
struct Foo {
    i: i32,
}

fn print(f: &Foo) {
    println!("{:p}", f);
}

#[test]
fn add_test() {
    let z = square(100, 200);
    let r = square::<f32>(4.2, 5.0);
}

fn square<T: Mul<T, Output=T>>(x: T, y: T) -> T {
    x * y
}

fn ya_iif(flag: bool) -> &'static str {
    if flag {
        "true"
    } else {
        "false"
    }
}

fn iif(flag: bool) -> &'static str {
    ["false", "true"][flag as usize]
}

#[test]
fn test_fn() {
    println!("{}", ya_iif(false));
    println!("{}", iif(true));
}

#[test]
fn test_fn_ref() {
    #![allow(unused)]
    trait Trait {
        fn f(self);
    }

    impl<T> Trait for fn(T) {
        fn f(self) {
            print!("1");
        }
    }

    impl<T> Trait for fn(&T) {
        fn f(self) {
            print!("2");
        }
    }

    let a: fn(t: i32) = |i: i32| {};
    let b: fn(_) = |_: &u8| {};
    let c: fn(&_) = |_: &u8| {};
    a.f();
    b.f();
    c.f();
}