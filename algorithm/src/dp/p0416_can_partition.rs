use crate::Solution;


impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum = nums.iter().sum::<i32>() as usize;
        if sum & 1 == 1 {
            return false;
        }
        let cap = sum >> 1;
        let len = nums.len();
        let mut dp = vec![vec![false; cap + 1]; len];

        for j in 0..dp[0].len() {
            dp[0][j] = nums[0] == j as i32;
        }

        for i in 0..dp.len() {
            dp[i][0] = true;
        }

        for i in 1..len {
            for j in 1..=cap {
                let n = nums[i] as usize;
                if j >= n {
                    dp[i][j] = dp[i - 1][j - n] || dp[i - 1][j]
                } else {
                    dp[i][j] = dp[i - 1][j]
                }
            }
        }
        //println!("{:?}", dp);
        dp[len-1][cap]
    }

    pub fn can_partition2(nums: Vec<i32>) -> bool {
        let sum = nums.iter().sum::<i32>();
        if sum & 1 == 1 {
            return false;
        }
        // -1 init 0 false 1 true
        let mut dp = vec![vec![-1; (sum / 2 + 1) as usize]; nums.len()];
        part(&nums, nums.len(), sum / 2, &mut dp)
    }
}

// nums[0..idx)是否可以填满sum
pub fn part(nums: &Vec<i32>, idx: usize, sum: i32, dp: &mut Vec<Vec<i32>>) -> bool {
    if sum == 0 {
        return true
    }
    if idx == 0 || sum < 0 {
        return false
    }
    if dp[idx-1][sum as usize] != -1 {
        return dp[idx-1][sum as usize] == 1
    }

    let r = part(nums, idx - 1, sum, dp) || part(nums, idx - 1, sum - nums[idx - 1], dp);
    dp[idx-1][sum as usize] = r as i32;
    return r
}

#[test]
fn test() {
    let v = vec![1, 5, 11, 5];
    let r1 = Solution::can_partition2(v.clone());
    let r2 = Solution::can_partition(v);
    println!("{}, {}", r1, r2)
}
