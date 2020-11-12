use std::cmp::max;
use crate::Solution;

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0; nums.len() + 1];
        let mut max_len = 0;
        for i in 0..nums.len() {
            if nums[i] == 1 {
                dp[i + 1] = nums[i] + dp[i];
                max_len = max(max_len, dp[i + 1]);
            }
        }
        max_len
    }
}