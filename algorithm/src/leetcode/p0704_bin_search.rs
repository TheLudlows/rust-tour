use crate::leetcode::common::Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        nums.binary_search(&target)
            .map_or(-1,|x| x  as i32)
    }
}