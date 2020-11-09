use std::cmp::Reverse;
use std::collections::BinaryHeap;

use crate::Solution;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut heap = BinaryHeap::with_capacity(k);
        for n in nums {
            if heap.len() < k {
                heap.push(Reverse(n));
            } else {
                if n < heap.peek().unwrap().0 {
                    heap.pop();
                    heap.push(Reverse(n));
                }
            }
        }
        heap.pop().unwrap().0
    }
}

#[test]
fn test() {}
