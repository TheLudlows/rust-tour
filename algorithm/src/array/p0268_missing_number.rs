use crate::Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let sum = (nums.len()-1)*nums.len()/2;
        sum - nums.iter().sum::<i32>()
    }
}