use crate::Solution;

/// 1. 额外空间，保存为0的i,j标，再次遍历，置为0
/// 2. 标记头部
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let x = matrix.len();
        let y = matrix[0].len();
        let mut x_flag = false;
        let mut y_flag = false;

        for i in 0..x {
            if matrix[i][0] == 0 {
                x_flag = true;
                break;
            }
        }
        for j in 0..y {
            if matrix[0][j] == 0 {
                y_flag = true;
                break;
            }
        }
        for i in 1..x {
            for j in 1..y {
                if matrix[i][j] == 0 {
                    matrix[0][j] = 0;
                    matrix[i][0] = 0;
                }
            }
        }

        for i in 1..x {
            if matrix[i][0] == 0 {
                for j in 1..y {
                    matrix[i][j] = 0;
                }
            }
        }

        for j in 1..y {
            if matrix[0][j] == 0 {
                for i in 1..x {
                    matrix[i][j] = 0;
                }
            }
        }

        if y_flag {
            for j in 0..matrix[0].len() {
                matrix[0][j] = 0
            }
        }
        if x_flag {
            for i in 0..matrix.len() {
                matrix[i][0] = 0
            }
        }
    }
}