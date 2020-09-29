use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use std::thread::JoinHandle;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        let mut workers = Vec::with_capacity(size);

        let (sender, receiver) = mpsc::channel();
        let rec = Arc::new(Mutex::new(receiver));
        for id in 0..size {
            workers.push(Worker::new(id, rec.clone()));
        }
        ThreadPool {
            workers,
            sender,
        }
    }
    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            if let Some(j) = worker.h.take() {
                j.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    h: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let t = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("Worker {} got a job; executing.", id);
                job();
            }
        });
        Worker {
            id,
            h: Option::Some(t),
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;
