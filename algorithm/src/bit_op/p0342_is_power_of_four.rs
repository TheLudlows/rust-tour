use crate::Solution;

impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        if n > 0 &&  (n & (n -1) == 0) {
            let x = (n as f32).sqrt() as i32;
            x*x == n && (x & (x - 1) == 0)
        } else {
            false
        }
    }
}

#[test]
fn test() {
    Solution::is_power_of_four(2);
}