use std::cmp::max;

/// dp[i] = max(dp[i-2]+nums[i],dp[i-1])
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut dp = vec![0; nums.len()];
        let mut max = 0;
        for i in 0..nums.len() {
            if i == 0 {
                dp[0] = nums[0]
            } else if i == 1 {
                dp[1] = std::cmp::max(nums[0], nums[1]);
            } else {
                dp[i] = std::cmp::max(dp[i - 1], dp[i - 2] + nums[i]);
            }
            if dp[i] > max {
                max = dp[i];
            }
        }
        max
    }
}