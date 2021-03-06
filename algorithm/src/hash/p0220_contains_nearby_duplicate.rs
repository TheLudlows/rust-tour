use std::cmp::min;
use std::collections::{BTreeSet};

use crate::Solution;

/// 暴力
impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
        let t64: i64 = t.into();
        for i in 0..nums.len() {
            for j in i + 1..min(i + (k as usize) + 1, nums.len()) {
                if (nums[i] as i64 - nums[j] as i64).abs() <= t64 {
                    return true;
                }
            }
        }
        return false;
    }
    pub fn contains_nearby_almost_duplicate2(nums: Vec<i32>, k: i32, t: i32) -> bool {
        let k = std::cmp::min(k as usize, nums.len());
        let mut set = BTreeSet::new();
        let t = t as i64;
        for i in 0..nums.len() {
            let num = nums[i] as i64;
            if set.range(num - t..=num + t).next().is_some() {
                return true
            }
            set.insert(num);
            if i >= k {
                set.remove(&(nums[i - k] as i64));
            }
        }
        false
    }
}

#[test]
fn test() {
    let v = vec![1,2,3,1];
    Solution::contains_nearby_almost_duplicate2(v, 3,0);
}
