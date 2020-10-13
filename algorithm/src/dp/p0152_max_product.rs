use std::cmp::{max, min};

use crate::Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut result = nums[0];
        let mut max_pro = nums[0];
        let mut min_pro = nums[0];
        for i in 1..nums.len() {
            let n = nums[i];
            let temp_max = max_pro;
            max_pro = max(max_pro * n, max(n, min_pro * n));
            min_pro = min(temp_max * n, min(n, min_pro * n));
            result = max(result, max_pro);
        }
        result
    }
}