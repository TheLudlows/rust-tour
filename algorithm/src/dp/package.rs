// 01背包
use crate::Solution;

impl Solution {
    fn max_value(c: i32, ws: Vec<i32>, vs: Vec<i32>) -> i32 {
        let len = c as usize;

        let mut dp: Vec<Vec<i32>> = vec![vec![0; len+1 as usize]; ws.len()];
        for j in 0..=len {
            if j as i32 >= ws[0] {
                dp[0][j] = vs[0];
            }
        }

        for i in 1..dp.len() {
            for j in 1..=len {
                dp[i][j] = dp[i - 1][j];
                if j as i32 >= ws[i]  {
                    dp[i][j] = std::cmp::max(dp[i][j], vs[i] + dp[i - 1][j - ws[i] as usize])
                }
            }
        }
        println!("{:?}", dp);
        dp[ws.len() - 1][len]
    }
}

#[test]
fn test() {
    let ws = vec![1, 2, 3];
    let vs = vec![6, 10, 12];
    let c = 5;
    let r = Solution::max_value(c, ws, vs);
    println!("{}", r);
}

// 完全背包问题