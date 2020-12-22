use std::cell::RefCell;
use std::sync::{mpsc, Arc, Barrier, Mutex, RwLock};
use std::thread;
use std::thread::{Builder, JoinHandle, LocalKey};
use std::time::Instant;
use std::time::{self, Duration};

#[test]
fn new_thread() {
    let b = Box::new(1);
    let h = thread::spawn(|| {
        println!("thread running");
        thread::sleep(Duration::from_secs(1));
        println!("thread run end");
    });
    h.join().unwrap();
    println!("main thread");
}

#[test]
fn move_test() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn( move|| {
        thread::sleep(Duration::from_secs(1));
        println!("Here's a vector: {:?}", v);
    });
    //println!("{:?}",vv);
    handle.join().unwrap();
}

#[test]
fn mpsc_test() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        let r = tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

#[test]
fn mpsc_test2() {
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

#[test]
fn lock_test() {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}

#[test]
fn arc_test() {
    let counter = Arc::new(Mutex::new(0));
    //let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = counter.clone();
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

#[test]
fn test() {
    // Declaring a Arc type data
    let data = Arc::new(Mutex::new(vec![2, 5, 6]));
    // Creating 3 threads and implementing lock
    for i in 0..3 {
        let data1 = data.clone();
        thread::spawn(move || {
            let mut data2 = data1.lock().unwrap();
            data2[0] += i;
            println!("Thread id :{:?}", i);
            println!("Data value :{:?}", data2[0]);
            println!("in")
        })
        .join()
        .unwrap();
    }
    thread::sleep(Duration::from_millis(100));
    println!("data:{:?}", data.lock().unwrap())
}

#[test]
fn test_rwLock() {
    let lock = RwLock::new(5);
    {
        let r1 = lock.read().unwrap();
        assert_eq!(*r1, 5);
        let mut w = lock.write().unwrap();
        *w += 1;
        assert_eq!(*w, 6);
    }
    {
        let r2 = lock.read().unwrap();
        assert_eq!(*r2, 5);
    }
}

#[test]
fn test_build() {
    let t = thread::Builder::new()
        .name("four".to_string())
        .stack_size(1024)
        .spawn(|| println!("hello"))
        .unwrap();

    t.join();
    print!("rust")
}

#[test]
fn thread_local() {
    thread_local!(static Local:RefCell<i32> = RefCell::new(1));
    Local.with(|v| {
        *v.borrow_mut() = 100;
    });

    thread::spawn(|| {
        Local.with(|v| {
            println!("in sub thread {:?}", v.borrow());
            *v.borrow_mut() = 200;
        });
    })
    .join();
    Local.with(|v| println!("{:?}", v.borrow()))
}

pub fn spawn_new<F, T>(f: F) -> JoinHandle<T>
where
    F: (FnOnce() -> T) + Send + 'static,
    T: Send + 'static,
{
    Builder::new().spawn(f).expect("failed to spawn thread")
}

#[test]
fn test_new() {
    let join = spawn_new(|| {
        println!("new thread");
    });
    join.join();
}

#[test]
fn test_barrier() {
    let barrier = Arc::new(Barrier::new(5));
    let mut joins = vec![];
    for i in 0..5 {
        let b = barrier.clone();
        let j = thread::spawn(move || {
            println!("wait {:?}", thread::current().id());
            b.wait();
            println!("pass");
        });
        joins.push(j);
    }
    for join in joins {
        join.join().unwrap();
    }
}

#[test]
fn test_sleep() {
    println!("{:?}", time::SystemTime::now());
    thread::sleep(Duration::from_secs(1));
    println!("{:?}", time::SystemTime::now());
}

#[test]
fn test_park() {
    let t = thread::Builder::new()
        .spawn(|| {
            println!("thread park");
            thread::park();
            println!("thread finish");
            thread::yield_now()
        }).unwrap();
    thread::sleep(Duration::from_secs(1));
    println!("un park");
    t.thread().unpark();
    t.join().unwrap();
}

#[test]
fn test_arc_modify () {
    let arc = Arc::new(String::from("hello"));
    for _ in 0..3 {
        let mut c = arc.clone();
        thread::spawn(move || {
            //c.push_str("word");
        });
    }
}

#[test]
fn test_mux () {
    let lock = Arc::new(Mutex::new(String::from("hello")));
    let mut v = vec![];
    for _ in 0..3 {
        let mut lock = lock.clone();
        let t = thread::spawn(move || {
            let mut strs = lock.lock().unwrap();
            strs.push_str(" world")
        });
        v.push(t);
    }

    for t in v {
        t.join().unwrap();
    }
    println!("{:?}", lock)
}

#[test]
fn test_mux_panic () {
    let lock = Arc::new(Mutex::new(String::from("hello ")));
    let mut v = vec![];
    for i in 0..3 {
        let mut lock = lock.clone();
        let t = thread::spawn(move || {
            if i == 0 {
                panic!("0 panic");
            }
            let mut strs = lock.lock().unwrap();
            strs.push_str(i.to_string().as_str())
        });
        v.push(t);
    }

    for t in v {
        match t.join() {
            Err(e) => {

            }
            Ok(()) => {

            }
        }
    }
    println!("{:?}", lock)
}