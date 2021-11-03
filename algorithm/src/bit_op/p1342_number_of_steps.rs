use crate::Solution;

impl Solution {
    pub fn number_of_steps(mut num: i32) -> i32 {
        let mut n = 0;
        while num != 0 {
            if num % 2 == 0 {
                num /= 2;
            } else {
                num -= 1;
            }
            n += 1;
        }
        n
    }
}