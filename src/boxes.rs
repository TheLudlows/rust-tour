use std::borrow::{Borrow, BorrowMut};
use std::ops::{Deref, DerefMut};

use crate::boxes::List::{Nil, Node};

pub enum List {
    Node(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

#[test]
fn my_box() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    print(&MyBox::new(String::from("a")));
}

fn print(name: &str) {
    println!("Hello, {}!", name);
}

#[test]
fn drop_test() {
    // 注意a b drop的顺序
    let a = CustomSmartPointer {
        data: String::from("aaa"),
    };
    let b = CustomSmartPointer {
        data: String::from("bbb"),
    };
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

#[test]
fn owner_test() {
    let a = Box::new("a");
    let b = Box::new("b".to_string());
    let c = *a;
    let d = *b;
    println!("{}", a);
    // println!("{}",b)
    println!("{}", c);
    println!("{}", d);
}

#[test]
fn mut_test() {
    let mut a: Box<i32> = Box::new(1);
    let x: &i32 = a.borrow();
    let y = a.as_ref();
}


// `T` 的泛型 trait。
trait DoubleDrop<T> {
    // 定义一个调用者的方法，接受一个额外的参数 `T`，但不对它做任何事。
    fn double_drop(self, _: T);
}

// 对泛型的调用者类型 `U` 和任何泛型类型 `T` 实现 `DoubleDrop<T>` 。
impl<T, U> DoubleDrop<T> for U {
    // 此方法获得两个传入参数的所有权，并释放它们。
    fn double_drop(self, _: T) {}
}
struct structFoo;
#[test]
fn test_trait() {
    let a = structFoo;
    let b = structFoo;
    a.double_drop(b);
}

#[test]
fn test_ref() {
    let b = Box::new(10);
    assert_eq!(10,*b.deref());
    assert_eq!(10,*b);
    //assert_eq!(&10,b);
    assert_eq!(&10,b.deref());
    assert_eq!(&10,&*b)
}