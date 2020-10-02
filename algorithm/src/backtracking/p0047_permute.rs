use crate::Solution;

///
impl Solution {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        let mut arr: Vec<i32> = vec![];
        let mut vis = vec![false; nums.len()];
        nums.sort();
        backtrace_2(&nums, &mut arr, &mut result, &mut vis);
        return result;
    }
}

fn backtrace_2(nums: &Vec<i32>, arr: &mut Vec<i32>, result: &mut Vec<Vec<i32>>, vis: &mut Vec<bool>) {
    if arr.len() == nums.len() {
        result.push(arr.clone());
    } else {
        for i in 0..nums.len() {
            if !vis[i] {
                if i > 0 && nums[i] == nums[i - 1] && vis[i - 1] == true {
                    continue;
                }
                vis[i] = true;
                arr.push(nums[i]);
                backtrace_2(nums, arr, result, vis);
                vis[i] = false;
                arr.pop();
            }
        }
    }
}

#[test]
fn test() {}