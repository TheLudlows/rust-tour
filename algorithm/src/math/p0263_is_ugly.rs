use crate::Solution;

/// 对能被2,3,5整除的数不断除2,3,5，最后剩1就是，剩其他就不是
impl Solution {
    pub fn is_ugly(mut num: i32) -> bool {
        if num < 1 { return false; };
        while num % 2 == 0 { num /= 2; }
        while num % 3 == 0 { num /= 3; }
        while num % 5 == 0 { num /= 5; }
        num == 1
    }
}