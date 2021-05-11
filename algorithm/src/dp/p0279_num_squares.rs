use crate::Solution;

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let seq = (n as f64).sqrt() as usize;
        let n = n as usize;
        let mut dp = vec![i32::MAX; n + 1];
        let mut seq_arr = vec![0; seq + 1];

        for i in 1..=seq {
            seq_arr[i] = i * i;
        }
        dp[0] = 0;
        for i in 1..=n {
            for j in 1..=seq {
                if i < seq_arr[j] {
                    break;
                }
                dp[i] = std::cmp::min(dp[i], dp[i - seq_arr[j]] + 1)
            }
        }
        dp[n + 1]
    }
    // 记忆搜索
    pub fn num_squares2(n: i32) -> i32 {
        let mut memo = vec![0;(n + 1) as usize];
        search(n as usize, &mut memo) as i32
    }

}
pub fn search(n:usize, memo:&mut Vec<usize>) -> usize {
    if n == 0 || memo[n] != 0 {
        return memo[n];
    }
    let mut i = 1;
    let mut ret = n;
    while i*i <= n {
        ret = std::cmp::min(ret, search(n -i*i, memo) + 1);
        i+=1;
    }
    memo[n] = ret;
    ret
}
#[test]
fn test() {
   let r = Solution::num_squares2(12);
    println!("{}", r)
}