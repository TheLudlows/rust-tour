use crate::Solution;

/// 指针 倒序遍历字符串
impl Solution {
    pub fn reverse_words(s: String) -> String {
        let s = s.trim().as_bytes();
        let (mut start, mut end) = (s.len(), s.len());
        let mut result = String::new();
        while start > 0 {
            // find end
            while s[start - 1] != b' ' {
                start -= 1;
            }
            //result.push_str([start..end]);
        }
        result
    }
}
