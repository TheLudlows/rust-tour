use crate::leetcode::common::Solution;
/// 
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        let mut arr: Vec<i32> = vec![];
        let mut vis = vec![false;nums.len()];
        backtrace(&nums, &mut arr, &mut result,&mut vis);
        return result;
    }

}

fn backtrace(nums: &Vec<i32>, arr: &mut Vec<i32>, result: &mut Vec<Vec<i32>>,vis:&mut Vec<bool>) {
    if arr.len() == nums.len() {
        result.push(arr.clone());
    } else {
        for i in 0..nums.len() {
            if !vis[i] {
                vis[i] = true;
                arr.push(nums[i]);
                backtrace(nums, arr, result,vis);
                vis[i] = false;
                arr.pop();
            }
        }
    }
}

#[test]
fn test() {}