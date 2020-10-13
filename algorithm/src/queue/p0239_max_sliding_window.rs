use std::collections::VecDeque;

use crate::Solution;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut queue = VecDeque::new();
        let mut right: usize = k as usize;
        let mut left:usize = 0;
        for i in 0..right {
            push_to_max_queue(&mut queue,nums[i]);
        }
        let mut result = vec![*queue.front().unwrap()];
        while right<nums.len() {
            if nums[left] == *queue.front().unwrap() {
                queue.pop_front();
            }
            push_to_max_queue(&mut queue,nums[right]);
            result.push(*queue.front().unwrap());
            right+=1;
            left+=1;
        }
        result
    }
}
#[inline]
fn push_to_max_queue(queue:&mut VecDeque<i32>,n:i32) {
    loop {
        if let Some(max) = queue.back() {
            if *max < n {
                queue.pop_back();
            }else {
                queue.push_back(n);
                return;
            }
        }else {
            queue.push_back(n);
            return;
        }
    }
}
#[test]
fn test() {

}