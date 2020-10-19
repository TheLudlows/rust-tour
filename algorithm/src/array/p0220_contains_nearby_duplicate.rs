use std::cmp::min;
use std::collections::BTreeSet;

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
        let k = k as usize;
        let t = t as i64;
        let mut bts: BTreeSet<i64> = BTreeSet::new();
        for i in 0..nums.len() {
            if i > k {
                bts.remove(&(nums[i - 1 - k] as i64));
            }
            if bts.range(nums[i] as i64 - t..=nums[i] as i64 + t).next().is_some() {
                return true;
            }
            bts.insert(nums[i] as i64);
        }
        false
    }
}