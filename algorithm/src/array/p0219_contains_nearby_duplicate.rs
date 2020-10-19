use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut map = HashMap::new();
        for i in 0..nums.len() {
            if map.contains_key(&nums[i]) {
                if i - map.get(&nums[i]).unwrap() <= k as usize{
                    return true;
                }
            }
            map.insert(&nums[i], i);
        }
        false
    }
}
#[test]
fn test() {

}