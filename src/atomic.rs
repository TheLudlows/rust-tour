use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::sync::atomic::Ordering::{Release, Acquire, Relaxed, SeqCst, AcqRel};

#[test]
fn test() {
    let atomic = Arc::new(AtomicUsize::new(0));
    let c1 = atomic.clone();
    let c2 = atomic.clone();

    let t1 = thread::spawn(move || {
        for i in 0..10000 {
            c1.store(c1.load(SeqCst)+1, SeqCst);
        }
    });
    let t2 = thread::spawn(move || {
        for i in 0..10000 {
            c2.store(c2.load(AcqRel)+1, SeqCst);
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();
    println!("{}",atomic.load(Relaxed));
}

#[test]
fn test_fetch_add() {
    let mut a:AtomicUsize = AtomicUsize::new(1);
    a.fetch_add(1,Relaxed);
}