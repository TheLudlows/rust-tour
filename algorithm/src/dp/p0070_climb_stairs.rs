use crate::Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }
        if n == 2 {
            return 2;
        }
        let n = n as usize;

        let mut dp = vec![0; n + 1];
        dp[0] = 0;
        dp[1] = 1;
        dp[2] = 2;
        for i in 3..=n {
            dp[i] = dp[i - 1] + dp[i - 2]
        }
        dp[n]
    }

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
