use crate::Solution;

impl Solution {
    pub fn swap_numbers(mut numbers: Vec<i32>) -> Vec<i32> {
        numbers[0] ^= numbers[1];
        numbers[1] ^= numbers[0];
        numbers[0] ^= numbers[1];
        numbers
    }
}

#[test]
fn test() {
    let a = 5;
    let b = 6;

    println!("{}", a^b);
}