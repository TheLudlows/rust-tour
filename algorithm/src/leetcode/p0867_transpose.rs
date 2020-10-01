use crate::leetcode::common::Solution;

impl Solution {
    pub fn transpose(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let x = a.len();
        let y = a[0].len();
        let mut result = vec![];
        for i in 0.. y {
            let mut new_vec = vec![];
            for j in 0..x {
                new_vec.push(a[j][i]);
            }
            result.push(new_vec);
        }
        result

    }
}