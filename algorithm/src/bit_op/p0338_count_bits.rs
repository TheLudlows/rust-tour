use crate::Solution;

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut ret = vec![0; (n + 1) as usize];
        for i in 1..=n as usize {
            if i % 2 == 0 {
                ret[i] = ret[i/2];
            } else {
                ret[i] = ret[i-1] + 1;
            }
        }
        ret
    }
}