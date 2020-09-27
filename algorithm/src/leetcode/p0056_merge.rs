use crate::leetcode::common::Solution;

/// 排序
/// Vec last元素的获取
impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut slice = intervals;
        let mut merge: Vec<Vec<i32>> = Vec::new();
        slice.sort_by(|a, b| a[0].cmp(&b[0]));
        for v in slice {
            let l = v[0];
            let r = v[1];
            if merge.is_empty() || merge.last().unwrap()[1] < l {
                merge.push(v);
            } else {
                merge.last_mut().unwrap()[1] = std::cmp::max(merge.last().unwrap()[1], r);
            }
        }
        merge
    }
}