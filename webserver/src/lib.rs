use std::thread;

pub struct ThreadPool{
    threads: Vec<thread::JoinHandle<_>>,
}
impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        ThreadPool{
            threads:Vec::with_capacity(size)
        }
    }
    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static {
    }
}