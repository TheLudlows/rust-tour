use crate::Solution;

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let pre = 1;
        for i in (0..digits.len()).rev() {
            let cur = digits[i] + pre;
            if cur < 10 {
                digits[i] = cur;
                return digits;
            } else {
                digits[i] = 0;
            }
        }
        digits.insert(0, 1);
        digits
    }
}