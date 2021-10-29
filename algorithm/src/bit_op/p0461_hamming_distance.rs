use crate::Solution;

impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        bit_count(x ^ y)
    }
}

pub fn bit_count(mut n: i32) -> i32 {
    let m1 = 0b01010101010101010101010101010101;
    let m2 = 0b00110011001100110011001100110011;
    let m4 = 0b00001111000011110000111100001111;
    let m8 = 0b00000000111111110000000011111111;
    let m16 = 0b00000000000000001111111111111111;
    n = (n & m1) + (n >> 1 & m1);
    n = (n & m2) + (n >> 2 & m2);
    n = (n & m4) + (n >> 4 & m4);
    n = (n & m8) + (n >> 8 & m8);
    n = (n & m16) + (n >> 16 & m16);
    n
}

#[test]
fn test() {
    println!("{}", 0b01010101010101010101010101010101)
}