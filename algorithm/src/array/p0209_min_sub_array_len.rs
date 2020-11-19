use std::cmp::min;
use crate::Solution;

/// double pointer
impl Solution {
    pub fn min_sub_array_len(s: i32, nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let (mut i, mut j) = (0, 0);
        let mut min_len = i32::max_value();
        let mut sum = 0;
        while j < nums.len() {
            sum += nums[j];
            while sum >= s {
                min_len = min(min_len, (j - i) as i32);
                sum -= nums[i];
                i += 1;
            }
            j+=1;
        }
        return if min_len == i32::max_value() {
            0
        } else {
            min_len + 1
        }
    }
}

#[test]
fn test() {
    let v = vec![1, 4, 4];
    let i = v.binary_search(&2);
    println!("{:?}",i);
    Solution::min_sub_array_len(4, v);
}