use crate::leetcode::common::Solution;

/// 最简单的动态规划了
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        if nums.len() == 1 {
            return nums[0];
        }
        let mut max = nums[0];
        let mut sum = max;
        for i in 1..nums.len() {
            if sum > 0 {
                sum += nums[i];
            } else {
                sum = nums[i];
            }

            if max < sum {
                max = sum;
            }
        }
        max
    }
}