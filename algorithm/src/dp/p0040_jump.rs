use std::cmp::max;

use crate::Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut end = 0;
        let mut cur_max = 0;
        let mut step = 0;
        for i in 0..nums.len() - 1 {
            cur_max = max(cur_max, i + nums[i] as usize);
            if i == end {
                // 刷新边界
                step += 1;
                end = cur_max as usize;
            }
        }
        step
    }
}
