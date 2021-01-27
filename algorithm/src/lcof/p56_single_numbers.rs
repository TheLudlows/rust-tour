use std::collections::HashMap;

use crate::Solution;

/// hash计数
impl Solution {
    pub fn single_numbers(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        for i in nums {
            let mut n = map.entry(i).or_insert(3);
            *n -= 1;
            if *n == 0 {
                map.remove(&i);
            }
        }

        for (i, _) in map {
           return i
        }
        return 0
    }
    /// 位运算 分组
    pub fn single_numbers2(nums: Vec<i32>) -> Vec<i32> {
        let mut map = HashMap::new();
        let mut res = vec![];
        for i in nums {
            let mut n = map.entry(i).or_insert(0);
            *n += 1;
        }

        for (i, n) in map {
            if n == 1 {
                res.push(i);
            }
        }
        res
    }
}

#[test]
fn test() {
    let r = 0 ^ 3;
    println!("{}", r)
}
