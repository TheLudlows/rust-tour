use std::ops::{Deref, DerefMut};
use crate::boxes::List::{Node, Nil};
use std::borrow::{BorrowMut, Borrow};

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
    let a = CustomSmartPointer { data: String::from("aaa") };
    let b = CustomSmartPointer { data: String::from("bbb") };
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
    let x:&i32 = a.borrow();
    let y = a.as_ref();
}
