use crate::leetcode::common::Solution;
use std::fs::read;

/// 1. 暴力枚举所有子字符串，逐个判断是不是回文 N^3
/// 2. 动态规划
///
///

impl Solution {
    // 暴力
    pub fn longest_palindrome(s: String) -> String {
        let bytes = s.as_bytes();
        let size = s.len();
        let mut max = 1;
        let mut start = 0;
        for i in 0..size {
            for j in i + 1..size {
                if j - i + 1 > max && is_huiwen(&bytes, i, j) {
                    start = i;
                    max = j - i + 1;
                }
            }
        }
        println!("{},{}",start,max);
        //std::str::from_utf8(&bytes[start..start + max]).unwrap().into_string()
        unsafe { String::from_utf8_unchecked(Vec::from(&bytes[start..start + max])) }
    }

}

fn is_huiwen(str: &[u8], mut left: usize, mut right: usize) -> bool {
    while left < right {
        if str[left] != str[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }
    true
}

#[test]
fn test() {
    let str ="abccdc".to_string();
    println!("{}",Solution::longest_palindrome(str))
}