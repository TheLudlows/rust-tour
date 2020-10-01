use std::cmp::min;

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let len = cost.len();
        let mut dp = vec![0; len];
        dp[0] = cost[0];
        dp[1] = cost[1];
        for i in 2..len {
            dp[i] = cost[i] + min(dp[i - 1], dp[i - 2]);
        }

        min(dp[len - 1], dp[len - 2])
    }
}