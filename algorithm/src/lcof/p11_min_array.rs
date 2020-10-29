use crate::Solution;

/// 只关注右边界
impl Solution {
    pub fn find_min(numbers: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, numbers.len() - 1);
        while l < r {
            let m = (l + r) / 2;
            if numbers[m] > numbers[r] {
                l = m + 1;
            } else {
                r = m
            }
        }
        numbers[l]
    }
}