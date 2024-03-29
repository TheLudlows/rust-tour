use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::{AcqRel, Relaxed, SeqCst};
use std::thread;

#[test]
fn test() {
    let atomic = Arc::new(AtomicUsize::new(0));
    let c1 = atomic.clone();
    let c2 = atomic.clone();

    let t1 = thread::spawn(move || {
        for i in 0..10000 {
            c1.store(c1.load(SeqCst) + 1, SeqCst);
        }
    });
    let t2 = thread::spawn(move || {
        for i in 0..10000 {
            c2.store(c2.load(SeqCst) + 1, SeqCst);
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();
    println!("{}", atomic.load(Relaxed));
}

#[test]
fn test_fetch_add() {
    let a: AtomicUsize = AtomicUsize::new(1);
    a.fetch_add(1, Relaxed);
}