use crate::Solution;
use std::collections::HashMap;

///  排序的str作为key
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut res = vec![];
        let mut map = HashMap::new();
        for s in strs {
            let mut key = s.clone().into_bytes();
            key.sort();
            let v = map.entry(key).or_insert(vec![]);
            v.push(s);
        }
        for x in map.values() {
            res.push(x.clone())
        }
        res
    }
}
#[test]
fn test() {}
