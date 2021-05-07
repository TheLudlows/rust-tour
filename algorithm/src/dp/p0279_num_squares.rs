use crate::Solution;

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let seq = (n as f64).sqrt() as usize;
        let n = n as usize;
        let mut dp = vec![i32::MAX; n+1];
        let mut seq_arr = vec![0;seq+1];

        for i in 1..=seq {
            seq_arr[i] = i*i;
        }
        dp[0] = 0;
        for i in 1..=n {
            for j in 1..=seq {
                if i < seq_arr[j] {
                    break;
                }
                dp[i] = std::cmp::min(dp[i], dp[i-seq_arr[j]] + 1)
            }
        }
        dp[n+1]
    }
}