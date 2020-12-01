use std::collections::VecDeque;
use futures::task;
use std::pin::Pin;
use std::future::Future;

fn main() {


}
struct MiniTokio {
    tasks: VecDeque<Task>,
}

type Task = Pin<Box<dyn Future<Output = ()> + Send>>;
