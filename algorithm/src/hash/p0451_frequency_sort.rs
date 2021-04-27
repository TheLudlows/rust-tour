use crate::Solution;
use std::collections::HashMap;
use std::collections::btree_map::Entry;

impl Solution {
    pub fn frequency_sort(s: String) -> String {

        let mut map = HashMap::new();
        for b in s.chars() {

            let buf = map.entry(b).or_insert(String::new());
            buf.push(b);
        }

        let mut v = map.into_iter().map(|e| e.1).collect::<Vec<String>>();
        v.sort_by(|e1,e2|e2.len().cmp(&e1.len()));
        let mut ret = String::new();

        v.into_iter().for_each(|e| ret.push_str(e.as_str()));
        ret
    }
}