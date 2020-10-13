use crate::Solution;

/// 读写双指针
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut read = 0;
        let mut write = 0;
        while read < nums.len() {
            if nums[read] != 0 {
                if read != write {
                    nums.swap(read, write);
                }
                write += 1;
            }
            read += 1;
        }
    }
}