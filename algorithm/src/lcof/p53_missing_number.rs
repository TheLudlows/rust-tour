use crate::Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let (mut i, mut j) = (0, nums.len());
        if i == j {
            return (nums[0] == 0) as i32;
        }
        while i < j {
            let mid = i + j >> 1;
            if mid == nums[mid] as usize {
                i = mid + 1;
            } else {
                j = mid;
            }
        }
        i as i32
    }
}

#[test]
fn test() {
    let v = vec![0, 1];
    Solution::missing_number(v);
}
