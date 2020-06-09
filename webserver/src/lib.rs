use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        let mut workers = Vec::with_capacity(size);

        let (sender,receiver) = mpsc::channel();
        for id in 0..size {
            workers.push(Worker::new(id,receiver));
        }
        ThreadPool {
            workers,
            sender
        }
    }
    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static {}
}

struct Worker {
    id: usize,
    h: thread::JoinHandle<Receiver<Job>>,
}

impl Worker {
    fn new(id: usize, receiver : mpsc::Receiver<Job>) -> Worker {
        let t = thread::spawn(|| receiver);
        Worker{
            id,
            h:t
        }
    }
}
struct Job;