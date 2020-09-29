use std::collections::VecDeque;

use crate::leetcode::common::Solution;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut stack = VecDeque::new();
        let chars = s.chars();
        let mut max = 0;
        let mut cur = 0;
        for c in chars.into_iter() {
            if c == ')' {
                let old = stack.back();
                old.map(|&oc|
                    if oc == '(' {
                        cur += 2;
                    } else {
                        cur = 0;
                    }
                );
            } else {
                let old = stack.back();
                old.map(|&oc| {
                    if oc == '(' {
                        cur = 0;
                    }
                });
            }
            stack.push_back(c);

            if cur > max {
                max = cur;
            }
        }
        max
    }
}

#[test]
fn test() {
    println!("{}", Solution::longest_valid_parentheses("()()()(())".to_string()));
}