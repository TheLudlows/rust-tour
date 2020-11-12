use crate::Solution;

impl Solution {
    pub fn word_break(s: String, mut word_dict: Vec<String>) -> bool {
        word_dict.sort();
        let mut dp = vec![false;s.len()+1];
        dp[0] = true;
        let s = s.as_str();
        for i in 1..=s.len() {
            for j in 0..i {
                dp[i] = dp[j] && word_dict.contains(&s[j..i].to_string());
            }
        }
        *dp.last().unwrap()
    }
}