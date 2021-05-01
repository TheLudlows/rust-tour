use std::collections::{HashMap, HashSet};

use crate::Solution;

/// 思路一：key num : value i
/// 思路二：滑动窗口
impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut map = HashMap::new();
        for i in 0..nums.len() {
            if map.contains_key(&nums[i]) {
                if i - map.get(&nums[i]).unwrap() <= k as usize {
                    return true;
                }
            }
            map.insert(&nums[i], i);
        }
        false
    }

    pub fn contains_nearby_duplicate2(nums: Vec<i32>, k: i32) -> bool {
        let k = std::cmp::min(k as usize, nums.len());
        let mut set = HashSet::new();

        for i in 0..nums.len() {
            if set.contains(&nums[i]) {
                return true
            }
            set.insert(nums[i]);
            if set.len() == k + 1 {
            //if i >= k {
                set.remove(&nums[i - k]);
            }
        }
        false
    }
}

#[test]
fn test() {}
