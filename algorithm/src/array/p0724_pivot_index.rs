use crate::Solution;

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let total = nums.iter().sum::<i32>();
        let mut sum = 0;

        for i in 0..nums.len() {
            if sum*2 == total-nums[i] {
                return i as i32;
            }
            sum +=nums[i];

        }
        return -1
    }
}