use crate::Solution;

impl Solution {
    pub fn can_permute_palindrome(s: String) -> bool {
        let mut v = vec![0; 128];
        for b in s.as_bytes() {
            v[*b as usize] += 1;
        }
        let mut count = 0;
        for i in v {
            if i % 2 != 0 && i > 0 {
                count += 1;
            }
        }
        count < 2
    }
}

#[test]
fn test() {
    Solution::can_permute_palindrome("code".to_string());
}