use crate::Solution;
use std::cmp::max;

impl Solution {
    pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut max_len = 0;
        for i in 0..nums1.len() {
            for j in 0..nums2.len() {
                let mut len = 0;
                while i + len < nums1.len() && j + len < nums2.len() && nums1[i + len] == nums2[j + len] {
                    len +=1;
                }
                max_len = std::cmp::max(max_len,len as i32)
            }
        }
        max_len
    }

    pub fn find_length2(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0;nums1.len()+1];nums2.len()+1];
        let mut max_len = 0;
        for i in (0..nums1.len()).rev(){
            for j in (0..nums2.len()).rev() {
                if nums1[i] == nums2[j] {
                    dp[i][j] = dp[i+1][j+1] +1;
                }
                max_len = std::cmp::max(max_len, dp[i][j]);
            }
        }
        max_len
    }
}