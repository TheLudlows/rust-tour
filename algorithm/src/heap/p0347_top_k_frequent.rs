use std::collections::{BinaryHeap, HashMap};

use crate::Solution;

/// map的使用
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        nums.iter().for_each(|n| {
            let count = map.entry(*n).or_insert(0);
            *count += 1;
        });

        let mut heap = BinaryHeap::new();

        for (n, v) in map {
            if heap.len() < k as usize {
                heap.push((-v, n));
            } else {
                if heap.peek().unwrap().0 > -v {
                    heap.pop();
                    heap.push((-v, n))
                }
            }
        }
        heap.iter().map(|(_, k)| *k).collect()
    }
}

#[test]
fn test() {}