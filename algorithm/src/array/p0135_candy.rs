use std::cmp::max;

use crate::Solution;

/// 两次遍历
impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let mut candy: Vec<i32> = vec![1; ratings.len()];
        let n = ratings.len();
        for i in 1..n {
            if ratings[i] > ratings[i - 1] {
                candy[i] = candy[i - 1] + 1;
            }
        }

        for i in 1..n {
            if ratings[n - i] < ratings[n - i - 1] {
                candy[n - i - 1] = max(candy[n - i] + 1, candy[n - i - 1]);
            }
        }
        candy.iter().sum()
    }
}

#[test]
fn test() {
    let v = vec![1, 3, 4, 5, 2];
    let r = Solution::candy(v);
    println!("{}", r);
}
