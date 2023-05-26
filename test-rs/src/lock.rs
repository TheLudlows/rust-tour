use std::{
    cell::{Cell, RefCell},
    sync::{atomic::{AtomicBool, Ordering}, Arc}, thread
};
#[derive(Debug)]
struct Lock<T> {
    lock: AtomicBool,
    data: RefCell<T>,
}
// SAFETY: 我们确信 Lock<T> 很安全，可以在多个线程中共享
unsafe impl<T> Sync for Lock<T> {}

impl<T> Lock<T> {
    fn new(data: T) -> Self {
        Self {
            lock: AtomicBool::new(false),
            data: RefCell::new(data),
        }
    }

    fn lock(&self, f : impl FnOnce(&mut T)) {
        while self.lock.compare_exchange(false, true, 
            Ordering::AcqRel, Ordering::Relaxed).is_err() {
            
        }
        f(&mut self.data.borrow_mut());
        self.lock.store(false, Ordering::Release);
    }

}

#[test]
fn test_lock() {
    let  l = Arc::new(Lock::new(32));
    l.lock(|i| {*i *=2;});
    println!("{:?}", l);
    let l1 = l.clone();
    let j  = thread::spawn(move ||{
        l1.lock(|i| {*i *=2;})
    });
    j.join().unwrap();
    print!("{:?}", l);
}
