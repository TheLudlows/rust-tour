use std::cmp::max;
use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let mut max_len = -1;
        let mut map = HashMap::new();
        let bytes = s.as_bytes();
        for i in 0..bytes.len() {
            if map.contains_key(&bytes[i]) {
                max_len = max(max_len, (i - map.get(&bytes[i]).unwrap() - 1) as i32);
            } else {
                map.insert(bytes[i], i);
            }
        }
        max_len
    }
}

#[test]
fn test() {}
