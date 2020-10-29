use crate::Solution;

/// 双指针指向头尾，不符合的数据 move，exchange
impl Solution {
    pub fn exchange(mut nums: Vec<i32>) -> Vec<i32> {
        if nums.is_empty() {
            return nums;
        }
        let mut i = 0;
        let mut j = nums.len() - 1;
        while i < j {
            if nums[i] % 2 == 1 {
                i += 1;
            } else if nums[j] % 2 == 0 {
                j -= 1;
            } else {
                nums.swap(i, j);
            }
        }
        nums
    }
}