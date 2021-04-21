use std::cmp::Reverse;
use std::collections::BinaryHeap;

// 数据流中位数
struct MedianFinder {
    max_heap: BinaryHeap<i32>,
    min_heap: BinaryHeap<Reverse<i32>>,

}

impl MedianFinder {
    fn new() -> Self {
        Self {
            max_heap: BinaryHeap::new(),
            min_heap: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        self.max_heap.push(num);
        if self.max_heap.len() - self.min_heap.len() > 1 {
            self.min_heap.push(Reverse(self.max_heap.pop().unwrap()));
        } else {
            if !self.min_heap.is_empty() && *self.max_heap.peek().unwrap() > self.min_heap.peek().unwrap().0 {
                self.min_heap.push(Reverse(self.max_heap.pop().unwrap()));
                self.max_heap.push(self.min_heap.pop().unwrap().0);
            }
        }
    }

    fn find_median(&self) -> f64 {
        if self.max_heap.len() == self.min_heap.len() {
            return (self.max_heap.peek().unwrap() + self.min_heap.peek().unwrap().0) as f64 / 2.0;
        } else {
            *self.max_heap.peek().unwrap() as f64
        }
    }
}

#[test]
fn test() {
    let mut finder = MedianFinder::new();
    finder.add_num(1);
    finder.add_num(2);
    println!("{}", finder.find_median());
    finder.add_num(3);
    println!("{}", finder.find_median());
}