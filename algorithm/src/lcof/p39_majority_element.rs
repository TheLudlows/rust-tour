use crate::Solution;

/// 一个数开始count=1，遇到相同的就加1，遇到不同的就减1，减到0就重新换个数开始计数，总能找到最多的那个

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let (mut count, len, mut most) = (0, nums.len(), nums[0]);
        for i in 0..len {
            count += if most == nums[i] { 1 } else { -1 };
            if count < 0 {
                most = nums[i];
                count = 0;
            }
        }
        most
    }
}
