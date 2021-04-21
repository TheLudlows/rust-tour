use crate::Solution;

impl Solution {
    pub fn is_straight(mut nums: Vec<i32>) -> bool {
        nums.sort();
        let mut joker = 0;

        for i in 0..nums.len() {
            if nums[i] == 0 {
                joker += 1;
            } else if i > 0 && nums[i - 1] == nums[i] {
                return false;
            }
        }
        (nums[nums.len() - 1] - nums[joker] - nums.len() as i32) < joker as i32
    }
}

#[test]
fn test() {
    let v = vec![10, 11, 0, 12, 6];
    Solution::is_straight(v);
}