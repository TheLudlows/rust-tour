use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn p(s: String) -> String {
        let mut v = vec![0; 26];
        s.as_bytes().iter().for_each(|&e| v[(e - b'a') as usize] += 1);

        v.into_iter().fil
        String::new()
    }
}

#[test]
fn test() {}