use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn min_set_size(arr: Vec<i32>) -> i32 {
        let mut map = HashMap::new();

        let mut rm = 0;
        for i in arr {
            let c = map.entry(i).or_insert(0);
            *c += 1;
        }

        let mut v = map.iter().collect::<Vec<(i32,i32)>>();

        v.sort_by(|k1,k2| -> k1.0.cmp(k2));
        let count = 0;
        for (k,n) in v {

        }
        1
    }
}