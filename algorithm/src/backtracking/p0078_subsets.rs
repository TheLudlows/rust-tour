use crate::Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut cur = vec![];
        trace(0, &mut result, &nums, &mut cur);
        result
    }
}

fn trace(start: usize, result: &mut Vec<Vec<i32>>, nums: &Vec<i32>, cur: &mut Vec<i32>) {
    result.push(cur.clone());

    for i in start..nums.len() {
        cur.push(nums[i]);
        trace(i + 1, result, nums, cur);
        cur.pop();
    }
}
