use std::sync::{Mutex, Condvar};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::atomic::{AtomicUsize, Ordering};

fn main() {
    println!("Hello, world!");
}

struct PoolData {
    name: Option<String>,
    receiver : Mutex<Receiver<Task>>,
    queue_count:AtomicUsize,
    active_count:AtomicUsize,
    max_thread_count:AtomicUsize,
    panic_count:AtomicUsize,
    stack_size:Option<usize>,
    lock:Mutex<()>,
    wait:Condvar
}

impl PoolData{
    fn has_word(&self) -> bool {
        self.queue_count.load(Ordering::AcqRel) > 0 || self.active_count.load(Ordering::AcqRel) > 0
    }
}

trait Runnable {
    fn call(self: Box<Self>);
}

impl<F> Runnable for F where F: FnOnce() {
    fn call(self: Box<Self>) {
        (*self)()
    }
}
type Task = Box<Runnable + Send>;

pub struct ThreadPool{
    sender : Sender<Task>,
    pool_data:PoolData
}
impl ThreadPool {
    fn new(thread_nums:usize) -> ThreadPool{
        Builder::new().thread_nums(thread_nums).build ()
    }

    fn execute<F>(&self, f:F) where F:FnOnce() + Send {
        self.sender.send(Box::new(f)).expect("send task failed");
        self.pool_data.queue_count.fetch_add(1,Ordering::SeqCst);
    }
    fn join(&self) {
        if !self.pool_data.has_word() {
            return;
        }
        self.pool_data.lock.lock().unwrap();
        while self.pool_data.has_word() {

        }

    }
}