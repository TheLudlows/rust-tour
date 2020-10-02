use crate::Solution;

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let x = obstacle_grid.len();
        let y = obstacle_grid[0].len();
        let mut dp = vec![vec![0; x]; y];
        if obstacle_grid[0][0] == 1 {
            return 0;
        }
        dp[0][0] = 1;
        for i in 0..x {
            for j in 0..y {
                if i == 0 && j == 0 {
                    continue;
                }
                if obstacle_grid[i][j] == 1 {
                    dp[i][j] = 0;
                    continue;
                }
                if i == 0 {
                    dp[i][j] = dp[i][j - 1];
                } else if j == 0 {
                    dp[i][j] = dp[i - 1][j];
                } else {
                    dp[i][j] = dp[i - 1][j] + dp[i][j - 1]
                }
            }
        }
        dp[x - 1][y - 1] as i32
    }
}