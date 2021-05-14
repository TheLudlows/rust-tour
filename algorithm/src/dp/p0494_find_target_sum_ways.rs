use crate::Solution;
use std::collections::HashMap;

// 二维扁平数组优化，
impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let mut memo = HashMap::new();
        find(&nums, target, nums.len(), &mut memo)
    }
}
fn find(nums: &Vec<i32>, target: i32, idx: usize, memo:&mut HashMap<i32,i32>) -> i32 {
    if idx == 0 && target == 0 {
        return 1;
    }
    if idx == 0 {
        return 0;
    }
    let ways = memo.get(&(target * nums.len() as i32 + idx as i32));
    if ways.is_some() {
        return *ways.unwrap();
    }
    let mut r = find(nums, target - nums[idx - 1], idx - 1, memo);
    r += find(nums, target + nums[idx - 1], idx - 1, memo);
    memo.insert(target * nums.len() as i32 + idx as i32, r);
    r
}

#[test]
fn test() {
    let nums= vec![1, 1, 1, 1, 1];
    let r = Solution::find_target_sum_ways(nums, 3);
    println!("{}", r);
}