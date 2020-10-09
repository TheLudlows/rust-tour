
use std::collections::BinaryHeap;
use crate::Solution;


impl Solution {
    pub fn get_least_numbers(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let len = arr.len();
        let mut maxHeap = BinaryHeap::from(arr);
        let mut result = vec![];
        for i in 0..len {
            if i > (len -k as usize)  {
                result.push(maxHeap.pop().unwrap())
            }
        }
        result
    }
}
#[test]
fn test() {

}