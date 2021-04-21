use std::cmp::max;

use crate::Solution;

impl Solution {
    pub fn max_value(grid: Vec<Vec<i32>>) -> i32 {
        let x = grid.len();
        let y = grid[0].len();
        let mut dp = vec![vec![0; y]; x];

        for i in 0..x {
            for j in 0..y {
                if i == 0 && j == 0 {
                    dp[i][j] = grid[i][j];
                } else if i > 0 && j == 0 {
                    dp[i][j] = dp[i - 1][j] + grid[i][j];
                } else if i == 0 && j > 0 {
                    dp[i][j] = dp[i][j - 1] + grid[i][j];
                } else {
                    dp[i][j] = max(dp[i - 1][j], dp[i][j - 1]) + grid[i][j]
                }
            }
        }
        dp[x - 1][y - 1]
    }
}