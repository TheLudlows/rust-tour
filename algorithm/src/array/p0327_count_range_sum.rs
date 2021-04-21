use crate::Solution;

impl Solution {
    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut sum = vec![nums[0] as i64; nums.len()];
        for i in 1..nums.len() {
            sum[i] = nums[i] as i64 + sum[i - 1];
        }
        let lower = lower as i64;
        let upper = upper as i64;

        let mut count = 0;
        for i in 0..nums.len() {
            if sum[i] >= lower && sum[i] <= upper {
                count += 1;
            }
            for j in 0..i {
                if sum[i] - sum[j] >= lower && sum[i] - sum[j] <= upper {
                    count += 1;
                }
            }
        }
        count
    }
}