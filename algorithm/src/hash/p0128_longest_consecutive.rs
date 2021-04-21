use std::cmp::max;
use std::collections::HashSet;

use crate::Solution;

/// set i-1不存在 说明是某个序列最小值，然后一直递增直到找不到
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut set = HashSet::new();
        nums.iter().for_each(|e| {
            set.insert(e);
        });
        let mut longest = 0;
        for i in set.iter() {
            if !set.contains(&(*i - 1)) {
                let mut cur = *i + 1;
                let mut count = 1;
                while set.contains(&cur) {
                    count += 1;
                    cur += 1;
                }
                longest = max(longest, count)
            }
        }
        longest
    }
}

#[test]
fn test() {}
