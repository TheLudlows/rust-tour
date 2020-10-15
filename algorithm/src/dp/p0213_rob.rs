use std::cmp::max;

use crate::Solution;

impl Solution {
    pub fn rob_2(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        } else if n == 1 {
            return nums[0];
        } else {
            max(rob_sub(&nums[0..n - 1]), rob_sub(&nums[1..n]))
        }
    }
}

fn rob_sub(nums: &[i32]) -> i32 {
    let mut first = 0;
    let mut second = 0;
    for i in 0..nums.len() {
        if i == 0 {
            first = nums[0]
        } else if i == 1 {
            second = max(nums[0], nums[1])
        } else {
            let temp = second;
            second = max(first + nums[i], second);
            first = temp;
        }
    }
    // 兼容长度为1
    max(first, second)
}

#[test]
fn test() {}