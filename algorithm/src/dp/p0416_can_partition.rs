use crate::Solution;

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum = nums.iter().sum::<i32>() as usize;
        if sum & 1 == 1 {
            return false
        }
        let half = sum >> 1;
        let len = nums.len();
        let mut dp = vec![vec![false; half + 1]; len + 1];
        dp[0][0] = true;
        for j in 1..=len {
            let x = nums[j - 1] as usize;
            for i in 0..=half {
                if i == 0 {
                    dp[j][i] = true;
                }
                if i >= x {
                    dp[j][i] = dp[j - 1][i - x] || dp[j - 1][i]
                } else {
                    dp[j][i] = dp[j - 1][i]
                }
            }
        }
        dp[len][half]
    }

    pub fn can_partition_2(nums: Vec<i32>) -> bool {
        let sum = nums.iter().sum::<i32>() as usize;
        if sum & 1 == 1 {
            return false
        }
        let half = sum >> 1;
        let len = nums.len();
        let mut dp = vec![false; half + 1];
        dp[0] = true;
        for j in 0..len {
            let x = nums[j] as usize;
            let mut i = half;
            while i >= x {
                dp[i] = dp[i] || dp[i - x];
                i -= 1;
            }
        }
        //println!("{:?}",dp);
        dp[half]
}

#[test]
fn test() {
    let v = vec![1, 5, 11, 5];
    Solution::can_partition_2(v);
}