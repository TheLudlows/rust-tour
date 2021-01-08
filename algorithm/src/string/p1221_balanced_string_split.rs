use crate::Solution;

impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let mut stack = Vec::new();
        let mut count = 0;
        for ch in s.chars() {
            if !stack.is_empty() && *stack.last().unwrap() != ch {
                stack.pop();
                if stack.is_empty() { count += 1; }
            } else {
                stack.push(ch);
            }
        }
        count
    }
}