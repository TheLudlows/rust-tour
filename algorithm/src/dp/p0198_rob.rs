use std::cmp::max;

use crate::Solution;

/// dp[i] = max(dp[i-2]+nums[i],dp[i-1])
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut dp = vec![0; nums.len()];
        for i in 0..nums.len() {
            if i == 0 {
                dp[0] = nums[0]
            } else if i == 1 {
                dp[1] = max(nums[0], nums[1]);
            } else {
                dp[i] = max(dp[i - 1], dp[i - 2] + nums[i]);
            }
        }
        dp[nums.len() - 1]
    }
}

impl Solution {
    pub fn rob_1(nums: Vec<i32>) -> i32 {
        let mut first = 0;
        let mut second = 0;

        for i in 0..nums.len() {
            if i == 0 {
                first = nums[0];
            } else if i == 1 {
                second = max(nums[0], nums[1]);
            } else {
                let temp = second;
                second = max(first + nums[i], second);
                first = temp;
            }
        }
        max(first, second)
    }
}