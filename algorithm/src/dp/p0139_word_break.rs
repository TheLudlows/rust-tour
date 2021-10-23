use std::collections::HashSet;

use crate::Solution;

// &str is slice
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let word_dict = word_dict.into_iter().collect::<HashSet<String>>();
        let s = s.as_str();
        let mut memo = vec![false; s.len() + 1];
        memo[0] = true;
        for i in 1..=s.len() {
            for j in 0..i {
                memo[i] |= memo[j] && word_dict.contains(&s[j..i]);
                if memo[i] {
                    break;
                }
            }
        }
        println!("{:?}", memo);
        memo[s.len()]
    }

    pub fn word_break2(s: String, word_dict: Vec<String>) -> bool {
        let word_dict = word_dict.into_iter().collect::<HashSet<String>>(); // hash或者二分
        let mut memo = vec![-1; s.len() + 1];
        find(&word_dict, s.as_str(), &mut memo)
    }
}

// s 是否有由word_dict构成
pub fn find(word_dict: &HashSet<String>, s: &str, memo: &mut Vec<i32>) -> bool {
    if s.len() == 0 {
        return true;
    }
    if memo[s.len()] != -1 {
        return memo[s.len()] == 1;
    }
    let mut ret = false;
    for i in (0..s.len()).rev() {
        ret |= word_dict.contains(&s[i..s.len()]) && find(word_dict, &s[0..i], memo)
    }
    memo[s.len()] = ret as i32;
    ret
}

#[test]
fn test_slice() {
    let s = "a".to_string();
    let word_dict = vec!["a".to_string()];
    assert_eq!(Solution::word_break(s, word_dict), true);
}