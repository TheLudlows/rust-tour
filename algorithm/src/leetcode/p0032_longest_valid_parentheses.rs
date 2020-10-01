use std::cmp::max;
use std::collections::VecDeque;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let len = s.len();
        let bytes = s.as_bytes();
        let mut max_v = 0;
        let mut dp: Vec<usize> = vec![0; len];
        for i in 1..len {
            if bytes[i] == ')' as u8 {
                if bytes[i - 1] == '(' as u8 {
                    if i == 1 {
                        dp[i] = 2;
                    } else {
                        dp[i] = dp[i - 2] + 2;
                    }
                } else {
                    if i - dp[i - 1] > 0 && bytes[i - dp[i - 1] - 1] == '(' as u8 {
                        if i - dp[i - 1] >= 2 {
                            dp[i] = dp[i - 1] + dp[i - dp[i - 1] - 2] + 2
                        } else {
                            dp[i] = dp[i - 1] + 2;
                        }
                    }
                }
                max_v = max(max_v, dp[i]);
            }
        }
        max_v as i32
    }
}

#[test]
fn test() {
    println!("{}", Solution::longest_valid_parentheses("()()()(())".to_string()));
}