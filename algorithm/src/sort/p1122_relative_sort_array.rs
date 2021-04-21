use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn relative_sort_array(mut arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let map = arr2.into_iter().enumerate().map(|(i, x)| (x, i)).collect::<HashMap<i32, usize>>();
        arr1.sort_by_key(|e| map.get(e).cloned().unwrap_or(1000 + *e as usize));
        arr1
    }
}