use std::cmp::min;

use crate::Solution;

/// dp[i]表示i最小需要的硬币数，dp[i] = min(dp[i],dp[i-coin] + 1);
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut dp = vec![i32::MAX; (amount + 1) as usize];
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

        if dp[amount] == i32::MAX {
            -1
        } else {
            dp[amount]
        }
    }

    pub fn coin_change2(coins: Vec<i32>, amount: i32) -> i32 {
        let mut memo = vec![0; (amount + 1) as usize];
        find(&coins, amount, &mut memo);
        println!("{:?}", memo);
        memo[(amount) as usize]
    }
}
// 在coins中寻中和为amount的最小需要coins个数
fn find(coins: &Vec<i32>, amount: i32, memo: &mut Vec<i32>) -> i32 {
    if amount < 0 {
        return -1;
    }
    if amount == 0 {
        return 0;
    }
    let idx = amount as usize;
    if memo[idx] != 0 {
        return memo[idx]
    }
    let mut ret = i32::MAX;
    for i in 0..coins.len() {
        let n = find(coins, amount - coins[i], memo);
        if n != -1 {
            ret = std::cmp::min(ret, n + 1);
        }
    }
    if ret == i32::MAX {
        ret =-1;
    }
    memo[idx] = ret;
    ret
}

#[test]
fn test() {
    let coins = vec![1,2,5];
    let r = Solution::coin_change(coins, 11);
    println!("{}", r);
}
