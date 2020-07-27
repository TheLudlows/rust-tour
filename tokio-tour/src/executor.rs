use std::sync::mpsc::Receiver;
use std::sync::Arc;

struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}