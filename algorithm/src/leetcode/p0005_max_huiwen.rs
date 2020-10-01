use std::fs::read;

use crate::Solution;

/// 1. 暴力枚举所有子字符串,逐个判断是不是回文 N^3
/// 2. 动态规划,
///

impl Solution {
    // 暴力
    pub fn longest_palindrome(s: String) -> String {
        if s.len() <= 1 {
            return s;
        }
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
        //std::str::from_utf8(&bytes[start..start + max]).unwrap().to_string()
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
    let str = "abccdcc".to_string();
    println!("{}", Solution::longest_palindrome(str.clone()));
    println!("{}", Solution2::longest_palindrome(str));
}

struct Solution2;

impl Solution2 {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() <= 1 {
            return s;
        }
        let s = s.as_bytes();
        let mut start = 0;
        let mut max = 1;

        let mut dp = vec![vec![false; s.len()]; s.len()];
        for i in 0..s.len() {
            dp[i][i] = true
        }
        for i in 1..s.len() {
            for j in 0..i {
                if &s[i] != &s[j] {
                    dp[j][i] = false;
                } else {
                    if i - j < 3 {
                        dp[j][i] = true;
                    } else {
                        dp[j][i] = dp[j + 1][i - 1]
                    }
                }
                if dp[j][i] && i - j + 1 > max {
                    max = i - j + 1;
                    start = j;
                }
            }
        }
        unsafe { String::from_utf8_unchecked(Vec::from(&s[start..start + max])) }
    }
}
