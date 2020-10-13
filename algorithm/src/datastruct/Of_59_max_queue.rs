use std::collections::VecDeque;

struct MaxQueue {
    queue: VecDeque<i32>,
    max_queue: VecDeque<i32>,
}

impl MaxQueue {
    fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            max_queue: VecDeque::new(),
        }
    }

    fn max_value(&self) -> i32 {
        *self.max_queue.front().unwrap_or(&-1)
    }

    fn push_back(&mut self, value: i32) {
        self.queue.push_back(value);
        loop {
            if let Some(old) = self.max_queue.back() {
                if *old < value {
                    self.max_queue.pop_back();
                } else {
                    self.max_queue.push_back(value);
                    break;
                }
            } else {
                break;
            }
        }
    }

    fn pop_front(&mut self) -> i32 {
        let num = self.queue.pop_back().unwrap_or(-1);
        if let Some(max) = self.max_queue.front() {
            if *max == num {
                self.max_queue.pop_front();
            }
        }
        num
    }
}

#[test]
fn test() {

}