use crate::Solution;
use std::cmp::min;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let x = grid.len();
        let y = grid[0].len();
        let mut dp = vec![vec![0; y]; x];
        for i in 0..x {
            for j in 0..y {
                if i == 0 && j == 0 {
                    dp[i][j] = grid[i][j];
                    continue;
                }
                if i == 0 {
                    dp[i][j] = grid[i][j] + dp[i][j - 1];
                } else if j == 0 {
                    dp[i][j] = grid[i][j] + dp[i - 1][j];
                } else {
                    dp[i][j] = grid[i][j] + min(dp[i][j - 1], dp[i - 1][j]);
                }
            }
        }
        dp[x - 1][y - 1]
    }
}
