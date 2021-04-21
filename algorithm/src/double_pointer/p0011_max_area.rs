use std::cmp::{max, min};

use crate::Solution;

///双指针，移动值较小的指针，因为较大的面积不可能会以这条为边
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0usize;
        let mut right = height.len() - 1;
        let mut result = 0;
        while left < right {
            let h = min(height[left], height[right]) as usize;
            result = max(result, h * (right - left));
            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }
        result as i32
    }
}

#[test]
fn test() {}
