use crate::Solution;

impl Solution {

    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        for i in 0..nums.len() {
            res^=nums[i];
            res^=i as i32;
        }
        res^=nums.len() as i32;
        res
    }
}