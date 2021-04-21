use std::cmp::max;

use crate::Solution;

impl Solution {
    pub fn maximum_gap(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        nums.windows(2)

            .fold(0, |m, x| max(x[1] - x[0], m))
    }
}