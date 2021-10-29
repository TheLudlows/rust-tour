use crate::Solution;

impl Solution {
    pub fn has_alternating_bits(n: i32) -> bool {
        let n = (n ^ n >> 1) + 1;
        n & (n - 1) == 0
    }
}