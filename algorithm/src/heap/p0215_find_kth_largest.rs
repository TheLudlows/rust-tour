use crate::Solution;
use std::collections::BinaryHeap;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut  heap = BinaryHeap::from(nums);
        for _ in 1..k {
            heap.pop();
        }
        heap.pop().unwrap()
    }
}
#[test]
fn test() {

}