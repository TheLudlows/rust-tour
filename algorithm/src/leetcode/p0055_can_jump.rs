use crate::leetcode::common::Solution;

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