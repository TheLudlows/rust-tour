use crate::Solution;

impl Solution {
    pub fn reverse_words1(s: String) -> String {
        s.split_whitespace()
            .collect::<Vec<&str>>()
            .join(" ")
    }
}