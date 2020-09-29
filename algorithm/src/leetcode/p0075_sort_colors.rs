use crate::leetcode::common::Solution;

/// 荷兰国旗问题 三指针
/// 若 nums[curr] = 0 ：交换第 curr个 和 第p0个 元素，并将指针都向右移。
/// 若 nums[curr] = 2 ：交换第 curr个和第 p2个元素，并将 p2指针左移 。
/// 若 nums[curr] = 1 ：将指针curr右移。

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut p0 = 0;
        let mut cur = 0;
        let mut p2 = nums.len()-1;

        while cur <= p2 {
            // p2 为0时，直接返回
            if p2 == 0 {
                return ;
            }
            if nums[cur] == 0 {
                exchange(nums, p0, cur);
                p0 += 1;
                cur += 1;
            } else if nums[cur] == 2 {
                exchange(nums, p2, cur);
                p2 -= 1
            } else {
                cur += 1;
            }
        }
    }
}

fn exchange(nums: &mut Vec<i32>, i: usize, j: usize) {
    let temp = nums[i];
    nums[i] = nums[j];
    nums[j] = temp;
}
#[test]
fn test() {
    let mut v = vec![0,1,2,2];
    Solution::sort_colors(&mut v);
    println!("{:?}",v)
}