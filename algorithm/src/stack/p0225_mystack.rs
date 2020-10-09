use std::collections::VecDeque;
#[derive(Default)]
pub struct MyStack {
    queue: VecDeque<i32>,
}
/// 每加入一个都都出队之前所有的元素，加入到队尾
impl MyStack {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn push(&mut self, x: i32) {
        self.queue.push_back(x);
        for _ in 0..self.queue.len() - 1 {
            let val = self.queue.pop_front().unwrap();
            self.queue.push_back(val);
        }
    }
    pub fn pop(&mut self) -> i32 {
        self.queue.pop_front().unwrap()
    }
    pub fn top(&self) -> i32 {
        *self.queue.front().unwrap()
    }
    pub fn empty(&self) -> bool {
        self.queue.is_empty()
    }
}