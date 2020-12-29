use crate::Solution;

impl Solution {
    pub fn modify_string(s: String) -> String {
        let dir = (b'a'..=b'z').collect::<Vec<u8>>();
        let mut s = s.into_bytes();
        for i in 0..s.len() {
            if s[i] == b'?' {
                let left = if i == 0 {
                    None
                } else{
                    Some(s[i-1])
                };
                let right = if i == s.len()-1 {
                    None
                } else{
                    Some(s[i+1])
                };
                s[i] = dir.clone().into_iter().find(|&x| Some(x) != left && Some(x) != right).unwrap();
            }
        }
        String::from_utf8(s).unwrap()

    }
}
#[test]
fn test_() {}