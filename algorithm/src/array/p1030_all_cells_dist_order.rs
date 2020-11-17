use std::cmp::max;
use crate::Solution;

impl Solution {
    pub fn all_cells_dist_order(r: i32, c: i32, r0: i32, c0: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        for i in 0..r {
            for j in 0..c {
                result.push(vec![i, j]);
            }
        }
        result.sort_by_key(|v1| (v1[0] - r0).abs() + (v1[1] - c0).abs());
        result
    }
}

#[test]
fn test() {}