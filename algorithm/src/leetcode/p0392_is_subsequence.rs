use crate::leetcode::common::Solution;
/// 双指针
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let sb = s.as_bytes();
        let tb = t.as_bytes();
        let (mut sp,mut tp) = (0,0);
        while sp < sb.len() && tp < tb.len() {
            if sb[sp] == tb[tp] {
                sp += 1;
            }
            tp += 1;
        }

        return sp == sb.len()
    }
}