use std::cell::RefCell;
use std::collections::HashMap;

pub trait MessageQueue {
    fn send(&self, msg: &str);
    fn println(&self);
}

struct MyMQ {
    queue: RefCell<Vec<String>>,
}

impl MessageQueue for MyMQ {
    fn send(&self, msg: &str) {
        self.queue.borrow_mut().push(String::from(msg))
    }

    fn println(&self) {
        for str in self.queue.borrow().iter() {
            println!("{}", str)
        }
    }
}

impl MyMQ {
    fn new() -> MyMQ {
        MyMQ {
            queue: RefCell::new(vec![]),
        }
    }
}

fn main() {
    let q = MyMQ::new();
    q.send("a");
    q.println();

    let x = RefCell::new(5);
    *(x.borrow_mut()) = 20;
    println!("{:?}", x)
}

#[test]
fn test_borrow() {
    let cell = RefCell::new(5);
    let a = cell.borrow_mut();
    //let b = cell.borrow();
}

#[test]
fn test() {
    let mut s = String::new();
    let a = &s;
    let b = &mut s;
}

#[test]
fn test_refcll() {
    let rc = RefCell::new(HashMap::new());
    rc.borrow_mut().insert("a", "b");

    let rc2 = rc.clone();
    println!("{:?}", rc2);
    rc.borrow_mut().insert("1", "2");
    println!("{:?}", rc2);
}