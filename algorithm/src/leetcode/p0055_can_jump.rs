use crate::leetcode::common::Solution;

/// 维护一个最远可达位置，遍历数组如果但前下表大于最远可达位置，那么就不符合
/// 最远可达位置 max(i)=i+num[i]，前提是当前i可达
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let size = nums.len();
        let mut rightmost = 0;
        for i in 0..size {
            if i > rightmost {
                return false;
            }
            rightmost = rightmost.max(i + nums[i] as usize);
            if rightmost >= size - 1 {
                return true;
            }
        }
        true
    }
}