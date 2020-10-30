use std::collections::HashMap;

use crate::Solution;

/// hash计数
impl Solution {
    pub fn single_numbers(nums: Vec<i32>) -> Vec<i32> {
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
    /// 位运算 分组
    pub fn single_numbers2(nums: Vec<i32>) -> Vec<i32> {
        let (mut a, mut b) = (0, 0);
        let mut dif = 0;
        for i in nums.iter() {
            dif ^= i;
        }
        let mut mask = 1;
        while dif & mask == 0 {
            mask = mask << 1;
        }

        for i in nums.iter() {
            if i & mask == 0 {
                a ^= i;
            } else {
                b ^= i;
            }
        }
        vec![a, b]
    }
}

#[test]
fn test() {
    let r = 0 ^ 3;
    println!("{}", r)
}
