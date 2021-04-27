use std::collections::HashMap;

use crate::Solution;

// 注意 abba [dog dog dog dog]
impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let words = s.split(' ').collect::<Vec<&str>>();

        if pattern.len() != words.len() {
            return false;
        }

        let mut word_map = HashMap::new();
        let mut pattern_map = HashMap::new();

        for (i, p) in pattern.chars().enumerate() {
            let target_word = pattern_map.entry(p).or_insert(words[i]);
            let target_pattern = word_map.entry(words[i]).or_insert(p);

            if *target_pattern != p || *target_word != words[i] {
                return false;
            }
        }
        true
    }
}
