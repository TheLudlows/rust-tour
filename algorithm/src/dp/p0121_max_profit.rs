use std::i32::MAX;

use crate::Solution;

/// 记录历史最低价格，逐个遍历价格数组，当差价大于最大利润是更新最大利润
impl Solution {
    pub fn max_profit3(prices: Vec<i32>) -> i32 {
        let mut min = MAX;
        let mut max_profit = 0;
        for i in prices {
            if i < min {
                min = i;
            }
            let p = i - min;
            if p > max_profit {
                max_profit = p;
            }
        }
        max_profit
    }
}
