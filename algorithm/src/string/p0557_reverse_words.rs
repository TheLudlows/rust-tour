use crate::Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_whitespace()
            .map(|x| x.chars().rev().collect())
            .collect::<Vec<String>>()
            .join(" ")
    }
}
