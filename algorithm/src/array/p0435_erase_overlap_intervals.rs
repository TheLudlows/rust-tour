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
}