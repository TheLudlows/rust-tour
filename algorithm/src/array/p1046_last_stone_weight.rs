use std::collections::BinaryHeap;

use crate::Solution;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::from(stones);
        loop {
            let large = heap.pop();
            let small = heap.pop();
            if large.is_none() {
                return 0;
            }
            if small.is_none() {
                return large.unwrap();
            }
            let diff = large.unwrap() - small.unwrap();
            if diff != 0 {
                heap.push(diff);
            }
        }
    }
}