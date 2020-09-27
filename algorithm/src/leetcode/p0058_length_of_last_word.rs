use crate::leetcode::common::Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let bytes = s.trim().as_bytes();
        let mut n = 0;
        let index = bytes.len()-1;
        for i in 0..bytes.len() {
            if bytes[index - i] == ' ' as u8 {
                return  n;
            } else{
                n+=1;
            }
        }
        n
    }
}