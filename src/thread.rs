use std::thread;
use std::time::Duration;
use std::sync::{mpsc, Mutex, Arc, RwLock};

#[test]
fn new_thread() {
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
    let handle = thread::spawn(move || {
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
            println!("Thread id :{:?}",i );
            println!("Data value :{:?}", data2[0]);
            println!("in")
        }).join().unwrap();
    }
    thread::sleep(Duration::from_millis(100));
    println!("data:{:?}",data.lock().unwrap())
}
#[test]
fn test_rwLock() {
    let lock = RwLock::new(5);

    // many reader locks can be held at once
    {
        let r1 = lock.read().unwrap();
        let r2 = lock.read().unwrap();
        assert_eq!(*r1, 5);
        assert_eq!(*r2, 5);
    } // read locks are dropped at this point

    // only one write lock may be held, however
    {
        let mut w = lock.write().unwrap();
        *w += 1;
        assert_eq!(*w, 6);
    }
}