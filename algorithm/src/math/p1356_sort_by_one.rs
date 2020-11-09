use crate::Solution;

impl Solution {
    pub fn sort_by_bits(mut arr: Vec<i32>) -> Vec<i32> {
        arr.sort();
        arr.sort_by(|x,y| x.count_ones().cmp(&y.count_ones()));
        arr
    }

    pub fn sort_by_bits2(mut arr: Vec<i32>) -> Vec<i32> {
        arr.sort_by_key(|x| (x.count_ones(),*x));
        arr
    }
}