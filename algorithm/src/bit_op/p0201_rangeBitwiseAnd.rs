use crate::Solution;

impl Solution {
    pub fn range_bitwise_and(mut left: i32, mut right: i32) -> i32 {
        let mut shift = 0;
        while left < right {
            left >>= 1;
            right >>= 1;
            shift += 1;
            println!("{},{}", left, right);
        }
        left << shift
    }
}

#[test]
fn test() {
    println!("{}", 1 >> 5);
    Solution::range_bitwise_and(5,7);
}