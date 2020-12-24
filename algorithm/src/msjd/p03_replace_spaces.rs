use crate::Solution;

impl Solution {
    pub fn replace_spaces(s: String, length: i32) -> String {
        let length = length as usize;
        let mut chars = s.chars();
        let mut res = String::new();
        for _ in 0..length {
            if let Some(next) = chars.next() {
                if next == ' ' {
                    res.push('%');
                    res.push('2');
                    res.push('0');
                } else {
                    res.push(next);
                }
            }
        }
        return res;
    }
}