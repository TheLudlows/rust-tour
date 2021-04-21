use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn min_set_size(arr: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        for i in arr.iter() {
            let c = map.entry(*i).or_insert(0);
            *c += 1;
        }

        let mut v: Vec<(i32, i32)> = map.into_iter().collect();
        v.sort_by(|k1, k2| k2.1.cmp(&k1.1));
        let mut count = 0;
        let mut rm = 0;
        for (_, v) in v {
            count += v;
            rm += 1;
            if count >= (arr.len() / 2) as i32 {
                break;
            }
        }
        rm
    }
}

#[test]
fn _test() {
    let r = Solution::min_set_size(vec![1, 1, 2, 2, 3]);
    println!("{}", r);
}