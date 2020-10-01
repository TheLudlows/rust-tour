

///

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let n = n as usize;
        let mut dp: Vec<i32> = vec![0; (n + 1)];
        dp[0] = 1;
        dp[1] = 1;
        for i in 2..n + 1 {
            // 内层循环计算i的数量
            for j in 1..=i {
                dp[i] += dp[i - j] * dp[j - 1];
            }
        }
        dp[n]
    }
}

#[test]
fn test() {
    println!("{}", Solution::num_trees(3));
}