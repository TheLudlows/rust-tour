use std::cmp::max;
use std::collections::HashMap;

use crate::Solution;

/// 双指针维护一个窗口，i表示将要加入的元素，如果i存在于窗口
/// 则更改窗口的起始地址

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.len() == 0 {
            return 0;
        }
        let mut max_len = 0;
        let bytes = s.as_bytes();
        let mut start = 0;
        let mut map = HashMap::new();
        for end in 0..s.len() {
            start = max(map.insert(&bytes[end], end).map_or(0, |n| n + 1), start);
            max_len = max(max_len, end - start + 1);
        }
        max_len as i32
    }

    pub fn length_of_longest_substring2(s: String) -> i32 {
        if s.len() == 0 {
            return 0;
        }
        let mut max_len = 0;
        let mut index = vec![0; 128];
        let mut start = 0;
        for (end, c) in s.chars().enumerate() {
            let c = c as usize;
            start = max(index[c], start);
            max_len = max(max_len, end - start + 1);
            index[c] = end + 1;
        }
        max_len as i32
    }
}

#[test]
fn test() {
    let n = Solution::length_of_longest_substring("abba".to_string());
    println!("{}", n);
}
