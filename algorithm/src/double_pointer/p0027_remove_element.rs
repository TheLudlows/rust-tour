use crate::Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.len() < 1 {
            return 0;
        }
        let mut i = 0;
        let mut j = 0;
        while j < nums.len() {
            if nums[i] != val {
                nums[j] = nums[i];
                i+= 1;
            }
            j += 1;
        }
        j as i32
    }
}
