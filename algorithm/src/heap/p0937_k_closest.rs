use std::collections::BinaryHeap;

use crate::Solution;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut heap = BinaryHeap::new();
        for v in points {
            let distance = v[0].pow(2) + v[1].pow(2);
            if heap.len() < k as usize {
                heap.push((distance, v));
            } else {
                let top = heap.peek().unwrap();
                if distance < top.0 {
                    heap.pop();
                    heap.push((distance, v));
                }
            }
        }
        heap.into_vec().into_iter().map(|t| t.1).collect()
    }
}

#[test]
fn test() {
    let r = (-4i32).pow(2);

    println!("{}", r);

    let vs = vec![vec![-5, 4], vec![-6, -5], vec![4, 6]];
    Solution::k_closest(vs, 2);
}