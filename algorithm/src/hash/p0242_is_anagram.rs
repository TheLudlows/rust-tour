use crate::Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut tmp_s = [0; 26];
        let mut tmp_t = [0; 26];
        s.bytes().for_each(|v| tmp_s[(v - b'a') as usize] += 1);
        t.bytes().for_each(|v| tmp_t[(v - b'a') as usize] += 1);

        tmp_s == tmp_t
    }
}