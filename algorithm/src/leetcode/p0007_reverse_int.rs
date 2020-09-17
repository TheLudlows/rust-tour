use crate::leetcode::common::Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {

        x.abs()
            .to_string()
            .chars().rev()
            .collect::<String>()
            .parse::<i32>()
            .unwrap_or(0) * x.signum()
    }
}