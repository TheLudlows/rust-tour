use crate::Solution;

/// 最简单的动态规划了
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max = nums[0];
        let mut sum = 0;
        for i in nums {
            sum = std::cmp::max(sum + i, i);
            max = std::cmp::max(max, sum);
        }
        max
    }
}
