use std::collections::HashSet;

use crate::Solution;

impl Solution {
    pub fn num_jewels_in_stones(j: String, s: String) -> i32 {
        let set = j.as_bytes().iter().collect::<HashSet<&u8>>();
        s.as_bytes().iter().filter(|b| set.contains(b)).count() as i32
    }
}

#[test]
fn test() {}
