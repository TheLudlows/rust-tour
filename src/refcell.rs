use std::cell::RefCell;

pub trait MessageQueue {
    fn send(&self, msg: &str);
    fn println(&self);
}
struct MyMQ{
    queue:RefCell<Vec<String>>
}
impl MessageQueue for MyMQ {
    fn send(& self, msg: &str) {
        self.queue.borrow_mut().push(String::from(msg))
    }

    fn println(&self) {
      for str in  self.queue.borrow().iter() {
          println!("{}",str)
      }
    }
}

impl MyMQ {
    fn new() -> MyMQ{
        MyMQ{
            queue:RefCell::new(vec![])
        }
    }
}

fn main() {
    let q = MyMQ::new();
    q.send("a");
    q.println();

    let x = RefCell::new(5);
    *(x.borrow_mut()) = 20;
    println!("{:?}", x)
}