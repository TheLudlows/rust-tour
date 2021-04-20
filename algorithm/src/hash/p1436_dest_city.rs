use crate::Solution;
use std::collections::{HashSet};

impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let mut set = HashSet::new();
        for i in 0..paths.len() {
            set.insert(&paths[i][0]);
        }
        for v in paths.iter() {
            if !set.contains(&v[1]) {
                return v[1].clone()
            }
        }
        String::new()
    }
}