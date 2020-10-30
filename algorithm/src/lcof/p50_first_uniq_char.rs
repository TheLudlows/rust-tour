use crate::Solution;

impl Solution {
    pub fn first_uniq_char(s: String) -> char {
        let mut cnt = [0; 26];

        for ch in s.bytes() {
            cnt[(ch - b'a') as usize] += 1;
        }
        for (_, ch) in s.bytes().enumerate() {
            if cnt[(ch - b'a') as usize] == 1 {
                return ch as char;
            }
        }
        ' '
    }
}
