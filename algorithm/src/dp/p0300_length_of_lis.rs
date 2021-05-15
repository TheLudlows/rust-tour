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
        let mut memo = vec![-1; nums.len()];
        let mut ret = 0;
        for i in 0..nums.len() {
            ret = std::cmp::max(ret,find(&nums, i, &mut memo));
        }
        ret
    }
}
// 以idx结尾的递增子序列最长长度, 初始化都为1
fn find(nums:&Vec<i32>, idx:usize, memo:&mut Vec<i32>) -> i32 {
    if memo[idx] != -1 {
        return memo[idx]
    }
    let mut ret = 1;
    for i in 0..idx {
        if nums[idx] > nums[i] {
           ret = std::cmp::max(ret, 1 + find(nums, i, memo));
       }
    }
    memo[idx] = ret;
    ret
}
#[test]
fn test() {
    let v = vec![10,9,2,5,3,7,101,18];
    let r = Solution::length_of_lis2(v);
    println!("{}", r)
}
