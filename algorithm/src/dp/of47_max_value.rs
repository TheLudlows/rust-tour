use crate::Solution;

impl Solution {
    pub fn max_value(grid: Vec<Vec<i32>>) -> i32 {
        let x = grid.len();
        let y = grid[0].len();
        let mut dp = vec![vec![0; y]; x];

        dp[0][0] = grid[0][0];
        for i in 1..x {
            dp[i][0] = grid[i][0] + dp[i-1][0];
        }
        for j in 1..y {
            dp[0][j] =  grid[0][j] + dp[0][j-1] ;
        }

        for i in 1..x {
            for j in 1..y {
                dp[i][j] = std::cmp::max(dp[i - 1][j], dp[i][j - 1]) + grid[i][j]
            }
        }
        dp[x - 1][y - 1]
    }
}