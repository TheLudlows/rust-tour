use std::cmp::min;

use crate::Solution;

/// double pointer slide window
///
/// 最长等于某个数  map + 前缀和 p0516
/// 不重复 map + 滑动窗口 
impl Solution {
    pub fn min_sub_array_len(s: i32, nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let (mut i, mut j) = (0, 0);
        let mut min_len = i32::MAX;
        let mut sum = 0;
        while j < nums.len() {
            sum += nums[j];
            while sum >= s {
                min_len = min(min_len, (j - i) as i32 + 1);
                sum -= nums[i];
                i += 1;
            }
            j += 1;
        }
        return if min_len == i32::MAX {
            0
        } else {
            min_len
        };
    }
}

#[test]
fn test() {
    let v = vec![1, 4, 4];
    let i = v.binary_search(&2);
    println!("{:?}", i);
    Solution::min_sub_array_len(4, v);
}