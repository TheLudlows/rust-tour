use crate::Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        if s.eq("0") {
            return 0;
        }
        let s = s.as_bytes();
        let mut dp = vec![0; s.len() + 1];
        dp[0] = 1;
        for i in 1..=s.len() {
            if s[i-1] != b'0' {
                dp[i] += dp[i-1];
            }
            if i > 1 && (s[i-2] == b'1' || (s[i-2] == b'2' && s[i-1] <= b'6')) {
                dp[i] += dp[i-2];
            }
        }

        dp[s.len()]
    }
}

#[test]
fn test() {
    let r = Solution::num_decodings("130".to_string());
    println!("{}", r);
}