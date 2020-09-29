use std::sync::{Arc, Condvar, Mutex};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

fn main() {
    let pool = Builder::new().build();
    println!("{:?}", pool.pool_data.max_thread_count);
    pool.execute(|| print!("hello"));
    pool.join();
}

struct PoolData {
    name: Option<String>,
    receiver: Mutex<Receiver<Task>>,
    queue_count: AtomicUsize,
    active_count: AtomicUsize,
    max_thread_count: AtomicUsize,
    panic_count: AtomicUsize,
    stack_size: Option<usize>,
    lock: Mutex<()>,
    condition: Condvar,
}

impl PoolData {
    fn has_word(&self) -> bool {
        self.queue_count.load(Ordering::Acquire) > 0 || self.active_count.load(Ordering::Acquire) > 0
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

pub struct ThreadPool {
    sender: Sender<Task>,
    pool_data: Arc<PoolData>,
}

impl ThreadPool {
    fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static {
        self.sender.send(Box::new(f)).expect("send task failed");
        self.pool_data.queue_count.fetch_add(1, Ordering::SeqCst);
    }
    fn join(&self) {
        if !self.pool_data.has_word() {
            return;
        }
        let mut lock = self.pool_data.lock.lock().unwrap();
        while self.pool_data.has_word() {
            lock = self.pool_data.condition.wait(lock).unwrap()
        }
    }
}

pub struct Builder {
    thread_name: Option<String>,
    thread_nums: Option<usize>,
    stack_size: Option<usize>,
}

impl Builder {
    fn new() -> Builder {
        Builder {
            thread_name: None,
            thread_nums: None,
            stack_size: None,
        }
    }
    fn thread_nums(mut self, thread_nums: usize) -> Builder {
        assert!(thread_nums > 0);
        self.thread_nums = Some(thread_nums);
        self
    }

    fn build(self) -> ThreadPool {
        let (sender, rev) = channel::<Task>();
        let thread_num = self.thread_nums.unwrap_or(num_cpus::get());
        let pool_data = Arc::new(PoolData {
            name: self.thread_name,
            receiver: Mutex::new(rev),
            queue_count: AtomicUsize::new(0),
            active_count: AtomicUsize::new(0),
            max_thread_count: AtomicUsize::new(thread_num),
            panic_count: AtomicUsize::new(0),
            stack_size: self.stack_size,
            lock: Mutex::new(()),
            condition: Default::default(),
        });
        for _ in 0..thread_num {
            generate_thread(pool_data.clone());
        }
        ThreadPool {
            sender,
            pool_data,
        }
    }
}

fn generate_thread(pool_data: Arc<PoolData>) {
    let mut thread_builder = thread::Builder::new();
    if let Some(name) = &pool_data.name {
        thread_builder = thread_builder.name(name.clone());
    }
    if let Some(stack_size) = &pool_data.stack_size {
        thread_builder = thread_builder.stack_size(stack_size.to_owned()); // clone
    }

    thread_builder.spawn(move || {
        loop {
            let thread_count = pool_data.active_count.load(Ordering::Acquire);
            let max_count = pool_data.max_thread_count.load(Ordering::Relaxed);
            if thread_count > max_count {
                break;
            }
            let receiver = pool_data.receiver.lock().expect("err");
            let result = receiver.recv();
            if result.is_err() {
                break;
            }
            let task = result.unwrap();
            pool_data.queue_count.fetch_sub(1, Ordering::SeqCst);
            pool_data.active_count.fetch_add(1, Ordering::SeqCst);
            task.call();
            pool_data.active_count.fetch_sub(1, Ordering::SeqCst);
            pool_data.condition.notify_all();
        }
    }).unwrap();
}