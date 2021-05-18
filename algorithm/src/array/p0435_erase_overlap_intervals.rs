use crate::Solution;

impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        if intervals.len() == 0 {
            return 0;
        }
        intervals.sort_by(|a, b| a[1].cmp(&b[1]));
        let mut last = &intervals[0];
        let mut count = 0;
        for i in 1..intervals.len() {
            let cur = &intervals[i];
            if cur[0] < last[1] {
                count += 1;
            } else {
                last = &intervals[i];
            }
        }
        count
    }
    // 排序之后就是最长上升子序列
    pub fn erase_overlap_intervals2(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_by(|e1, e2| e1[0].cmp(&e2[0]));
        let mut ret = 0;
        let mut memo = vec![-1; intervals.len()];
        for i in 0..intervals.len() {
            ret = std::cmp::max(ret, find(&intervals, i, &mut memo))
        }
        intervals.len() as i32 - ret
    }
}
fn find(nums: &Vec<Vec<i32>>, idx: usize, memo: &mut Vec<i32>) -> i32 {
    let mut ret = 1;
    if memo[idx] != -1 {
        return memo[idx];
    }
    for i in 0..=idx {
        if nums[i][1] <= nums[idx][0] {
            ret = std::cmp::max(ret, 1 + find(nums, i, memo));
        }
    }
    memo[idx] = ret;
    ret
}