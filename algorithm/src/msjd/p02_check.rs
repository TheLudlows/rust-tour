use crate::Solution;

impl Solution {
    pub fn check_permutation(s1: String, s2: String) -> bool {
        let mut v1 = s1.into_bytes();
        v1.sort();
        let mut v2 = s2.into_bytes();
        v2.sort();
        v1 == v2
    }
}