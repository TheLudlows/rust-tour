use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        let mut res = vec![];
        for s in words {
            if isomorphic(&s, &pattern) {
                res.push(s);
            }
        }
        res
    }
}

pub fn isomorphic(s: &String, t: &String) -> bool {
    let mut map1 = HashMap::new();
    let mut map2 = HashMap::new();

    let s = s.as_bytes();
    let t = t.as_bytes();
    if s.len() != t.len() {
        return false;
    }

    for i in 0..s.len() {
        let v = map1.entry(s[i]).or_insert(t[i]);
        if *v != t[i] {
            return false;
        }
        let v = map2.entry(t[i]).or_insert(s[i]);
        if *v != s[i] {
            return false;
        }
    }
    true
}

#[test]
fn test() {
    let r = isomorphic(&"abc".to_string(), &"abb".to_string());
    println!("{}", r)
}