// 01背包
use crate::Solution;

impl Solution {
    fn max_value(c: i32, ws: Vec<i32>, vs: Vec<i32>) -> i32 {
        let len = c as usize;

        let mut dp: Vec<Vec<i32>> = vec![vec![0; len + 1 as usize]; ws.len()];
        for j in 0..=len {
            if j as i32 >= ws[0] {
                dp[0][j] = vs[0];
            }
        }

        for i in 1..dp.len() {
            for j in 1..=len {
                dp[i][j] = dp[i - 1][j];
                if j as i32 >= ws[i] {
                    dp[i][j] = std::cmp::max(dp[i][j], vs[i] + dp[i - 1][j - ws[i] as usize])
                }
            }
        }
        println!("{:?}", dp);
        dp[ws.len() - 1][len]
    }

    fn max_value2(c: i32, ws: Vec<i32>, vs: Vec<i32>) -> i32 {
        let mut memo = vec![vec![-1; (c + 1) as usize]; ws.len() + 1];
        find(c, &ws, &vs, ws.len(), &mut memo)
    }
}

pub fn find(c: i32, ws: &Vec<i32>, vs: &Vec<i32>, idx: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
    if c <= 0 {
        return 0;
    }
    if idx == 0 {
        return 0;
    }
    if memo[idx][c as usize] != -1 {
        return memo[idx][c as usize];
    }
    let mut ret = find(c, ws, vs, idx - 1, memo);
    if c >= ws[idx - 1] {
        ret = std::cmp::max(ret, vs[idx - 1] + find(c - ws[idx - 1], ws, vs, idx - 1, memo));
    }
    memo[idx][c as usize] = ret;
    ret
}

#[test]
fn test() {
    let ws = vec![1, 2, 3];
    let vs = vec![6, 10, 12];
    let c = 5;
    let r = Solution::max_value2(c, ws, vs);
    println!("{}", r);
}

// 完全背包问题,多次选用，和01背包的递归搜索方式有些不太一样