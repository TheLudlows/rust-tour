use crate::Solution;

impl Solution {
    pub fn search_range2(a: Vec<i32>, tgt: i32) -> Vec<i32> {
        if a.is_empty() {
            return vec![-1, -1];
        }
        let l = search_left(&a, tgt);
        if l < 0 {
            return vec![-1, -1];
        }
        let r = search_right(&a, tgt);
        return vec![l, r];
    }
}

fn search_left(nums: &Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;
    while left < right {
        let mid = (left + right) >> 1;
        if nums[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    if nums[left] == target {
        left as i32
    } else {
        -1
    }
}

fn search_right(nums: &Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;
    while left < right {
        let mid = (left + right + 1) >> 1;
        if nums[mid] > target {
            right = mid - 1;
        } else {
            left = mid;
        }
    }
    if nums[left] == target {
        (left) as i32
    } else {
        -1
    }
}
