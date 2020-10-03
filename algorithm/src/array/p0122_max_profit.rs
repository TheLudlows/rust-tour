use crate::Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut p = 0;
        for i in 1.. prices.len() {
            if prices[i] > prices[i-1] {
                p +=prices[i] - prices[i-1];
            }
        }
        p
    }
}