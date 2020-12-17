use std::cmp::max;

use crate::Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let mut dp = vec![vec![-1; 2]; prices.len()];
        dp[0][0] = 0;
        dp[0][1] = -prices[0];

        for i in 1..prices.len() {
            dp[i][0] = max(dp[i - 1][0], dp[i - 1][1] + prices[i] - fee);
            dp[i][1] = max(dp[i - 1][1], dp[i - 1][0] - prices[i]);
        }
        // println!("{:?}",dp);
        dp[prices.len() - 1][0]
    }

    pub fn max_profit2(prices: Vec<i32>, fee: i32) -> i32 {
        let len = prices.len();
        let mut has = -prices[0];
        let mut nHas = 0;

        for i in 1..len {
            let old = has;
            has = max(has, nHas - prices[i]);
            nHas = max(nHas, old + prices[i] - fee);
        }
        nHas
    }
}