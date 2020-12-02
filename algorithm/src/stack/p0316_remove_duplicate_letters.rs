use crate::Solution;
use std::collections::{HashSet, HashMap};

impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut result = String::new();
        let mut map = HashMap::new();
        s.chars().enumerate().for_each(|(i,c)| {map.insert(c,i);});
        let mut set:HashSet<char> = HashSet::new();
        for (i,c) in s.chars().enumerate() {
            if !set.contains(&c) {
                while !result.is_empty() && result.chars().last().unwrap() > c && *map.get(&result.chars().last().unwrap()).unwrap() > i {
                    set.remove(&result.pop().unwrap());
                }
                result.push(c);
                set.insert(c);
            }
        }
        result
    }
}

#[test]
fn test() {
    let r = Solution::remove_duplicate_letters("bcabc".to_string());
    println!("{}",r)
}
