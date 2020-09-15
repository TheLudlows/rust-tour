use crate::leetcode::common::Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        let mut arr: Vec<i32> = vec![];
        Solution::backtrace(&nums, &mut arr, &mut result);
        return result;
    }

    fn backtrace(nums: &Vec<i32>, arr: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if arr.len() == nums.len() {
            result.push(arr.clone());
        } else {
            for i in nums {
                if !arr.contains(i) {
                    arr.push(*i);
                    Solution::backtrace(nums, arr, result);
                    arr.pop();
                }
            }
        }
    }
}

#[test]
fn test() {}