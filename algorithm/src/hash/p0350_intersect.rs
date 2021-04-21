use std::collections::HashMap;

use crate::Solution;

/// hash 计数
impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut count_map = HashMap::new();
        let mut res = vec![];
        for i in nums1 {
            let n = count_map.entry(i).or_insert(0);
            *n += 1;
        }
        for i in nums2 {
            if let Some(count) = count_map.get_mut(&i) {
                if *count == 0 {
                    continue;
                }
                *count -= 1;
                res.push(i);
            }
        }
        res
    }
}
