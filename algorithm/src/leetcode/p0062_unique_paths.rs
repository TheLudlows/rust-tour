

/// 动态规划
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut dp = vec![vec![0; n as usize]; m as usize];
        let x = m as usize;
        let y = n as usize;
        for i in 0..x {
            for j in 0..y {
                if i == 0 {
                    dp[i][j] = 1;
                } else if j == 0 {
                    dp[i][j] = 1;
                } else {
                    dp[i][j] = dp[i - 1][j] + dp[i][j - 1]
                }
            }
        }
        dp[x - 1][y - 1]
    }
}

#[test]
fn test() {}