use std::collections::VecDeque;

use crate::Solution;
// 单调队列
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if nums.is_empty() {
            return vec![];
        }
        let mut queue = VecDeque::new();
        let mut right: usize = k as usize;
        let mut left: usize = 0;
        for i in 0..right {
            push_to_max_queue(&mut queue, i, &nums);
        }
        let mut result = vec![];
        result.push(nums[queue[0]]);
        while right < nums.len() {
            if left >= queue[0] {
                queue.pop_front();
            }
            push_to_max_queue(&mut queue, right, &nums);
            result.push(nums[queue[0]]);
            right += 1;
            left += 1;
        }
        result
    }
}

#[inline]
fn push_to_max_queue(queue: &mut VecDeque<usize>, index: usize, nums: &Vec<i32>) {
    loop {
        if let Some(&id) = queue.back() {
            if nums[id] < nums[index] {
                queue.pop_back();
            } else {
                queue.push_back(index);
                return;
            }
        } else {
            queue.push_back(index);
            return;
        }
    }
}

#[test]
fn test() {}
