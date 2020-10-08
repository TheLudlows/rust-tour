use std::cmp::max;
use crate::Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut dp = vec![1;nums.len()];
        let mut result = 0;
        for i in 0..nums.len() {
            for  j in 0..i {
                if nums[i] > nums[j] {
                    dp[i] = max(dp[i],dp[j] + 1)
                }
            }
            result = max(result,dp[i]);
        }
        result
    }
}