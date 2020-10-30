use crate::Solution;

/// 指针 倒序遍历字符串
impl Solution {
    pub fn reverse_words1(s: String) -> String {
        let s = s.trim().to_string();
        let (mut start, mut end) = (s.len(), s.len());
        let mut result = String::new();
        // todo
        result
    }
}
#[test]
fn test() {

}
impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_whitespace()
            .collect::<Vec<&str>>()
            .join(" ")
    }
}