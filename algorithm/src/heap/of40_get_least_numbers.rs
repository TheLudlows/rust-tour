use std::collections::BinaryHeap;

use crate::Solution;

/// 最小->大顶堆
impl Solution {
    pub fn get_least_numbers(arr: Vec<i32>, k: i32) -> Vec<i32> {
        if k == 0 {
            return vec![];
        }
        let k = k as usize;
        let mut max_heap = BinaryHeap::with_capacity(k);

        for i in arr {
            if max_heap.len() < k {
                max_heap.push(i);
            } else {
                if i < *max_heap.peek().unwrap() {
                    max_heap.pop();
                    max_heap.push(i);
                }
            }
        }
        max_heap.into_iter().collect::<Vec<i32>>()
    }
}

#[test]
fn test() {}
