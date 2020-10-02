use crate::Solution;

impl Solution {
    pub fn ways_to_step(n: i32) -> i32 {
        if n == 1 { return 1; };
        if n == 2 { return 2; };
        if n == 3 { return 4; };

        let mut dp: Vec<usize> = vec![0; (n + 1) as usize];
        dp[1] = 1;
        dp[2] = 2;
        dp[3] = 4;
        for i in 4..dp.len() {
            dp[i] = (dp[i - 1] + dp[i - 2] + dp[i - 3]) % 1000000007;
        }
        dp[n as usize] as i32
    }
}

#[test]
fn test() {}