use std::cell::{Cell, RefCell};
use std::sync::{Arc, Mutex};
use std::thread;

#[test]
fn cell_test() {
    struct Foo {
        b: Cell<i32>,
        c: Cell<String>,
    }

    let f = Foo {
        b: Cell::new(20),
        c: Cell::new("c".to_string()),
    };
    assert_eq!(20, f.b.get());
    f.b.set(30);
    assert_eq!(30, f.b.get());
    f.c.set("ccc".to_string())
}


#[test]
fn test_sync() {
    let c = Arc::new(Mutex::new(RefCell::new(1)));
    let cc = c.clone();
    thread::spawn(move || {
        let x = cc.lock().unwrap();
        let mut xx = x.borrow_mut();
        *xx +=1;
    }).join().unwrap();

    println!("{:?}",c);
}