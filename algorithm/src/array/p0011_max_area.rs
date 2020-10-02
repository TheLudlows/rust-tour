use std::cmp::min;
use crate::Solution;

///双指针，移动值较小的指针，因为较大的面积不可能会以这条为边
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0usize;
        let mut right = height.len() - 1;
        let mut max = min(height[left] * right as i32, height[right] * right as i32);
        while left < right {
            let temp = min(height[left] * (right - left) as i32, height[right] * (right - left) as i32);
            if temp > max {
                max = temp;
            }
            if &height[left] < &height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }
        max
    }
}

#[test]
fn test() {}