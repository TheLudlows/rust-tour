use crate::Solution;
/// @todo
impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let amount = (amount + 1) as usize;
        let mut dp = vec![0; amount];
        for i in 1..amount {
            for j in 0..coins.len() {
                let coin = coins[j] as usize;
                if i >= coin {
                    dp[i] += dp[i - coin];
                }
            }
        }
        dp[amount - 1]
    }
}

#[test]
fn test() {
    let v = vec![1, 2, 5];
    Solution::change(5, v);
}
