use std::collections::BinaryHeap;

use crate::Solution;

/// 最小->大顶堆
impl Solution {
    pub fn get_least_numbers(arr: Vec<i32>, k: i32) -> Vec<i32> {
        if k == 0 {
            return vec![]
        }
        let k = k as usize;
        let mut maxHeap = BinaryHeap::with_capacity(k);

        for i in arr {
            if maxHeap.len() < k {
                maxHeap.push(i);
            } else {
                if i < *maxHeap.peek().unwrap() {
                    maxHeap.pop();
                    maxHeap.push(i);
                }
            }
        }
        let mut result = vec![];
        for _ in 0..k {
            result.push(maxHeap.pop().unwrap())
        }
        result
    }
}

#[test]
fn test() {}