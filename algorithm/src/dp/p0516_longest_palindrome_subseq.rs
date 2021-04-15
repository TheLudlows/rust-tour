use crate::Solution;
use std::cmp;

impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut dp = vec![vec![0;bytes.len()];bytes.len()];

        for i in 0..bytes.len() {
            dp[i][i] = 1;
        }
        let mut longest = 1;
        for j in 1..bytes.len() {
            for i in 0..j {
                if bytes[i] == bytes[j] {
                    dp[i][j] = 2 + dp[i+1][j-1];
                } else {
                    dp[i][j] = cmp::max(dp[i][j-1],dp[i+1][j]);
                }
                longest = cmp::max(longest,dp[i][j]);
            }
        }
        longest
    }
}
#[test]
fn test() {
    let r = Solution::longest_palindrome_subseq("bbbab".to_string());
    println!("{}",r)
}

