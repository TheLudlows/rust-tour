use crate::Solution;

/// 切片法
impl Solution {
    pub fn reverse_left_words(s: String, n: i32) -> String {
        let n = n as usize  % s.len() ;
        let s1 = &s[..n];
        let s2 = &s[n..];
        String::from(s2) + &s1

    }
}
#[test]
fn test( ) {

}