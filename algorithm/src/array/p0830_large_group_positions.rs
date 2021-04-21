use crate::Solution;

impl Solution {
    fn large_group_positions(s: String) -> Vec<Vec<i32>> {
        let s = s.into_bytes();
        let (mut i, n) = (0, s.len());
        let mut ret = vec![];
        while i < n {
            let start = i;
            while i < n && s[i] == s[start] {
                i += 1;
            }
            if i - start >= 3 {
                ret.push(vec![start as i32, i as i32 - 1]);
            }
        }
        ret
    }
}

#[test]
fn test() {
    Solution::large_group_positions("aaa".to_string());
}