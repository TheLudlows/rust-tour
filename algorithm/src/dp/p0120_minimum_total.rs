use std::cmp::min;

use crate::Solution;

impl Solution {
    pub fn minimum_total(mut triangle: Vec<Vec<i32>>) -> i32 {
        for i in 1..triangle.len() {
            for j in 1..i {
                triangle[i][j] += min(triangle[i - 1][j], triangle[i - 1][j - 1])
            }
            triangle[i][0] += triangle[i - 1][0];
            triangle[i][i] += triangle[i - 1][i - 1];
        }
        *triangle.last().unwrap().iter().min().unwrap()
    }
}