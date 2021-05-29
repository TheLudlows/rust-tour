use std::cmp;

use crate::Solution;
use std::fs::read;

impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut dp = vec![vec![0; bytes.len()]; bytes.len()];

        for i in 0..bytes.len() {
            dp[i][i] = 1;
        }
        let mut longest = 1;
        for i in (1..bytes.len()).rev() {
            for j in i + 1..bytes.len() {
                if bytes[i] == bytes[j] {
                    dp[i][j] = 2 + dp[i + 1][j - 1];
                } else {
                    dp[i][j] = cmp::max(dp[i][j - 1], dp[i + 1][j]);
                }
                longest = cmp::max(longest, dp[i][j]);
            }
        }
        longest
    }

    pub fn longest_palindrome_subseq2(s: String) -> i32 {
        let mut memo = vec![vec![-1; s.len() - 1]; s.len() - 1];
        find(s.as_bytes(), 0, s.len() - 1, &mut memo)
    }
}

fn find(s: &[u8], i: usize, j: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
    if i > j {
        return 0;
    }
    if i == j {
        return 1;
    }
    if memo[i][j] != -1 {
        return memo[i][j];
    }
    let mut ret;
    if s[i] == s[j] {
        ret = 1 + find(s, i + 1, j - 1, memo);
    } else {
        ret = std::cmp::max(find(s, i, j - 1, memo), find(s, i + 1, j, memo));
    }
    memo[i][j] = ret;
    ret
}

#[test]
fn test() {
    let r = Solution::longest_palindrome_subseq("bbbab".to_string());
    println!("{}", r)
}

