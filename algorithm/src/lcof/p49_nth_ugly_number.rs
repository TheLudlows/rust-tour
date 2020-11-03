use std::cmp::min;
use crate::Solution;

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let mut ugly_numbers = vec![1; n as usize];
        let (mut i2, mut i3, mut i5) = (0, 0, 0);
        for i in 1..(n as usize) {
            let (x2, x3, x5) = (
                2 * ugly_numbers[i2],
                3 * ugly_numbers[i3],
                5 * ugly_numbers[i5],
            );
            let ugly = min(x2, min(x3, x5));
            ugly_numbers[i] = ugly;
            if ugly == x2 {
                i2 += 1;
            }
            if ugly == x3 {
                i3 += 1;
            }
            if ugly == x5 {
                i5 += 1;
            }
        }
        ugly_numbers[(n - 1) as usize]
    }
}