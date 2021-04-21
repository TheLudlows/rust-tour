use std::cmp::max;
use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn longest_subsequence(arr: Vec<i32>, dif: i32) -> i32 {
        let mut map = HashMap::new();
        let mut max_len = 1;
        for i in 0..arr.len() {
            let n = *map.get(&(arr[i] - dif)).unwrap_or(&0) + 1;
            max_len = max(max_len, n);
            map.insert(arr[i], n);
        }
        max_len
    }
}

#[test]
fn test() {
    let v = vec![1, 2, 3, 4];
    Solution::longest_subsequence(v, 1);
}