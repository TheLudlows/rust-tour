use std::collections::VecDeque;

struct MyCircularQueue {
    data: VecDeque<i32>,
    size: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularQueue {
    /** Initialize your data structure here. Set the size of the queue to be k. */
    fn new(k: i32) -> Self {
        Self {
            size: k,
            data: VecDeque::with_capacity(k as usize),
        }
    }

    /** Insert an element into the circular queue. Return true if the operation is successful. */
    fn en_queue(&mut self, value: i32) -> bool {
        if self.data.len() == self.size as usize {
            //self.data.pop();
            return false;
        }
        self.data.push_back(value);
        true
    }

    /** Delete an element from the circular queue. Return true if the operation is successful. */
    fn de_queue(&mut self) -> bool {
        if self.data.len() == 0 {
            false
        } else {
            self.data.pop_front();
            true
        }
    }

    /** Get the front item from the queue. */
    fn front(&mut self) -> i32 {
        *self.data.front().unwrap_or(&-1)
    }

    /** Get the last item from the queue. */
    fn rear(&mut self) -> i32 {
        *self.data.back().unwrap_or(&-1)
    }

    /** Checks whether the circular queue is empty or not. */
    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /** Checks whether the circular queue is full or not. */
    fn is_full(&self) -> bool {
        self.data.len() == self.size as usize
    }
}

/**
 * Your MyCircularQueue object will be instantiated and called as such:
 * let obj = MyCircularQueue::new(k);
 * let ret_1: bool = obj.en_queue(value);
 * let ret_2: bool = obj.de_queue();
 * let ret_3: i32 = obj.front();
 * let ret_4: i32 = obj.rear();
 * let ret_5: bool = obj.is_empty();
 * let ret_6: bool = obj.is_full();
 */
#[test]
fn test() {}