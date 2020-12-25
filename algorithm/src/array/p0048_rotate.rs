use crate::Solution;
use std::mem::{swap, replace};

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        for i in 0..n {
            for j in 0..i {
                //swap(&mut matrix[i][j],  &mut matrix[j][i])
            }
        }
        for i in 0..n {
            for j in 0..(n / 2) {
                matrix[i].swap(j, n - 1 - j);
            }
        }
    }
}