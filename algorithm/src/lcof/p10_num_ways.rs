use crate::Solution;

impl Solution {
    pub fn num_ways(n: i32) -> i32 {
        if n == 0 {
            return 1;
        } else if n <= 2 {
            return n;
        }
        let mut s1 = 1;
        let mut s2 = 2;

        for _ in 3..=n {
            let temp = s2;
            s2 = (s1 + s2) % 1000000007;
            s1 = temp;
        }
        s2
    }
}
