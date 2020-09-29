use crate::leetcode::common::Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let x = x as i64;
        let mut left = 0;
        let mut right = x + 1;
        while left < right - 1 {
            let mid = (left + right) / 2;
            let sqr = mid * mid;
            if x == sqr {
                return mid as i32;
            } else if x > sqr {
                left = mid;
            } else {
                right = mid;
            }
        }
        left as i32
    }
}

#[test]
fn test() {
    let r = Solution::my_sqrt(10);
    println!("{}", r);
}