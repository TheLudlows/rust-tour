use std::collections::{HashMap, HashSet};

use crate::Solution;

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut map = HashMap::new();
        for i in arr {
            let count = map.entry(i).or_insert(0);
            *count += 1;
        }

        let mut set = HashSet::new();
        for (_, count) in map {
            if !set.insert(count) {
                return false;
            }
        }
        true
    }
}

#[test]
fn test() {}
