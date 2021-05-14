use std::cmp::max;

use crate::Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut dp = vec![1; nums.len()];
        let mut result = 0;
        for i in 0..nums.len() {
            for j in 0..i {
                if nums[i] > nums[j] {
                    dp[i] = max(dp[i], dp[j] + 1)
                }
            }
            result = max(result, dp[i]);
        }
        result
    }
    pub fn length_of_lis2(nums: Vec<i32>) -> i32 {
        find(&nums, nums.len())
    }
}
fn find(nums:&Vec<i32>, idx:usize) -> i32 {
    if idx == 0 {
        return 0;
    }
    let mut ret = 0;
    for i in 0..idx {
        ret = std::cmp::max(ret, find(nums, i));

        if nums[idx-1] > nums[i] {
           ret = std::cmp::max(ret, 1 + find(nums, i));
       }
    }
    ret
}
#[test]
fn test() {
    let v = vec![10,9,2,5,3,7,101,18];
    let r = Solution::length_of_lis2(v);
    println!("{}", r)
}
