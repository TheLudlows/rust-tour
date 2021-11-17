use std::cell::{Cell, RefCell};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};
use std::thread;
use std::thread::Thread;

#[derive(Debug)]
struct Lock<T> {
    own: AtomicBool,
    data: RefCell<T>,
}
unsafe impl<T> Sync for Lock<T> {}
impl<T> Lock<T> {
    pub fn new(t: T) -> Self {
        Self { own: AtomicBool::new(false), data: RefCell::new(t) }
    }
    pub fn lock(&self, f : fn(&mut T)) {
        while self.own.compare_exchange(false, true, Acquire, Relaxed).is_err() {
        }
        f(&mut self.data.borrow_mut());
        self.own.store(false, Release);
    }
}
#[test]
fn test_lock() {
    let lock = Arc::new(Lock::new(10));
    let l1 = lock.clone();
    let h1 = thread::spawn(move|| {
        l1.lock(|n| *n+=3);
    });
    let l2 = lock.clone();

    let h2 = thread::spawn(move|| {
        l2.lock(|n| *n+=3);
    });
    h1.join().unwrap();
    h2.join().unwrap();

    println!("{:?}", lock);
}