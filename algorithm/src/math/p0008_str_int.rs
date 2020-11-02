use crate::Solution;

impl Solution {
    pub fn my_atoi(str: String) -> i32 {
        let mut negative = false;
        let mut res = 0i64;
        for (i, ch) in str.trim().chars().enumerate() {
            if ch == '+' && i == 0 { continue; }
            if ch == '-' && i == 0 {
                negative = true;
                continue;
            }
            if !ch.is_digit(10) { break; }
            res = 10i64 * res + ch.to_digit(10).unwrap() as i64;
            if negative {
                if -res < i32::min_value() as i64 {
                    return i32::min_value();
                }
            } else {
                if res > i32::max_value() as i64 {
                    return i32::max_value();
                }
            }
        }

        if negative {
            -res as i32
        } else {
            res as i32
        }
    }
}