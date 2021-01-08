use crate::Solution;
use std::cmp::max;
impl Solution {
    pub fn max_power(s: String) -> i32 {
        let s = s.as_bytes();
        let mut last = s[0];
        let mut counter = 1;
        let mut r = counter;
        for i in 1..s.len() {
            if s[i] == last {
                counter+=1;
                r = max(counter,r)
            } else {
                last = s[i];
                counter =1;
            }

        }
        r

    }
}