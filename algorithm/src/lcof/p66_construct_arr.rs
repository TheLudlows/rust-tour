use crate::Solution;

impl Solution {
    pub fn construct_arr(a: Vec<i32>) -> Vec<i32> {
        if a.is_empty() {
            return vec![];
        }
        let mut result = vec![1; a.len()];

        for i in 1..a.len() {
            result[i] = a[i - 1] * result[i - 1]
        }
        let mut r = 1;
        for i in (0..a.len()).rev() {
            result[i] = result[i] * r;
            r = a[i] * r;
        }
        result
    }
}