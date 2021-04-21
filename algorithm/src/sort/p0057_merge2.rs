use std::cmp::max;

use crate::Solution;

/// 上一题目相同思路
impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut slice = intervals;
        slice.push(new_interval);
        let mut merge: Vec<Vec<i32>> = Vec::new();
        slice.sort_by(|a, b| a[0].cmp(&b[0]));
        for v in slice {
            let l = v[0];
            let r = v[1];
            if merge.is_empty() || merge.last().unwrap()[1] < l {
                merge.push(v);
            } else {
                merge.last_mut().unwrap()[1] = max(merge.last().unwrap()[1], r);
            }
        }
        merge
    }
}
