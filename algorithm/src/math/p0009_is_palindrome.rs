use crate::Solution;

/// 求余数 计算出逆序的x 然后比较
impl Solution {
    pub fn is_palindrome1(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let mut rem = 0;
        let mut y = 0;
        let mut quotient = x;
        while quotient != 0 {
            rem = quotient % 10;
            quotient /= 10;
            y = y * 10 + rem;
        }
        return y == x;
    }
}