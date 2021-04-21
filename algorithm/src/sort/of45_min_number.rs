use crate::Solution;

impl Solution {
    pub fn min_number(nums: Vec<i32>) -> String {
        let mut v = nums.iter().map(|e| e.to_string()).collect::<Vec<String>>();
        v.sort_by(|x, y| format!("{}{}", x, y).cmp(&format!("{}{}", y, x)));
        v.join("")
    }
}