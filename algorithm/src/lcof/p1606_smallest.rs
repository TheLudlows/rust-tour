use std::cmp::min;

use crate::Solution;

impl Solution {
    pub fn smallest_difference(mut a: Vec<i32>, mut b: Vec<i32>) -> i32 {
        a.sort();
        b.sort();
        let (mut i, mut j) = (0, 0);
        let mut min_dif = i64::MAX;
        println!("{:?}", a);
        println!("{:?}", b);

        while i < a.len() && j < b.len() {
            min_dif = min(min_dif, (a[i] as i64 - b[j] as i64).abs());
            if a[i] > b[j] {
                j += 1
            } else if a[i] < b[j] {
                i += 1;
            } else {
                return 0;
            }
        }

        min_dif as i32
    }
}

#[test]
fn test_abs() {
    //-2147483648, 1]
    //[0, 2147483647]
    println!("{}", i32::MIN);
    let v1 = vec![-2147483648, 1];
    let v2 = vec![0, 2147483647];
    Solution::smallest_difference(v1, v2);
    println!("{}", (-1i32).abs())
}