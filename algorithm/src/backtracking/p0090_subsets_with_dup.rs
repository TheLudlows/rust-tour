use crate::Solution;

///  sort 去重
impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut cur = vec![];
        nums.sort();
        trace(0, &mut result, &nums, &mut cur);
        result
    }
}

fn trace(start: usize, result: &mut Vec<Vec<i32>>, nums: &Vec<i32>, cur: &mut Vec<i32>) {
    result.push(cur.clone());
    for i in start..nums.len() {
        if i > start && nums[i] == nums[i - 1] {
            continue;
        }
        cur.push(nums[i]);
        trace(i + 1, result, nums, cur);
        cur.pop();
    }
}
