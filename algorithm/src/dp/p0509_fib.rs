use crate::Solution;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        let r = (0..n).fold((0u64, 1), |(p1, p2), _| (p2, p1 + p2)).0;
        r as i32
    }
}