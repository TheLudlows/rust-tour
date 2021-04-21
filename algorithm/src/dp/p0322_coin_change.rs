use std::cmp::min;

use crate::Solution;

/// dp[i]表示i最小需要的硬币数，dp[i] = min(dp[i],dp[i-coin] + 1);
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut dp = vec![amount + 1; (amount + 1) as usize];
        dp[0] = 0;
        let amount = amount as usize;
        for i in 1..=amount {
            for j in 0..coins.len() {
                let coin = coins[j] as usize;
                if i >= coin {
                    dp[i] = min(dp[i], dp[i - coin] + 1);
                }
            }
        }

        if dp[amount] == (amount + 1) as i32 {
            -1
        } else {
            dp[amount]
        }
    }
}

#[test]
fn test() {}
