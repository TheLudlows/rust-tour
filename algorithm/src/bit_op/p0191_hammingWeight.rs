use crate::Solution;

impl Solution {
    pub fn hammingWeight(mut n: u32) -> i32 {
        let mut ret = 0;
        while n != 0 {
            ret += 1;
            n = n & (n - 1);
        }
        ret
    }
}