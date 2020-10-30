use std::collections::HashSet;

use crate::Solution;

impl Solution {
    pub fn find_repeat_number(nums: Vec<i32>) -> i32 {
        let mut d = HashSet::new();
        for i in nums.iter() {
            if d.contains(i) {
                return *i;
            }
            d.insert(*i);
        }
        -1
    }

    pub fn find_repeat_number2(nums: Vec<i32>) -> i32 {
        let mut arr = vec![-1; nums.len()];
        for i in nums {
            if arr[i as usize] == i {
                return i;
            }
            arr[i as usize] = i;
        }
        -1
    }

    /// 原地交换
    pub fn find_repeat_number3(mut nums: Vec<i32>) -> i32 {
        for i in 0..nums.len() {
            let mut j = nums[i] as usize;
            while i != j {
                if nums[i] == nums[j] {
                    return nums[i];
                }
                nums.swap(i, j);
                j = nums[i] as usize;
            }
        }
        -1
    }
}
