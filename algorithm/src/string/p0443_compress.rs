use crate::Solution;
use std::convert::TryFrom;

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut r = 0;
        let mut w = 0;
        while r < chars.len() {
            let i = r;
            while chars[r] == chars[i] && r < chars.len() {
                r+=1;
            }
            if r > i + 1 {
                let count = format!("{}",r - i);
                for c in count.chars() {
                    chars[w] = c;
                    w += 1;
                }
            }
        }
        w as i32
    }
}