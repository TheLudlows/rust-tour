use std::cmp::max;

use crate::Solution;

impl Solution {
    pub fn max_score_sightseeing_pair(a: Vec<i32>) -> i32 {
        /// a[i] + i + a[j]-j
        let mut max_i = a[0];
        let mut result = 0;
        for j in 1..a.len() {
            result = max(result, max_i + a[j] - j as i32);
            max_i = max(max_i, a[j] + j as i32);
        }
        result
    }
}