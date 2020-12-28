use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
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
            let v = map2.entry(t[i]).or_insert( s[i]);
            if *v != t[i] {
                return false;
            }
        }
        true
    }
}

#[test]
fn test() {
    Solution::is_isomorphic("aa".to_string(),"ab".to_string());
}