use crate::Solution;

/// usize 越界判断
impl Solution {
    pub fn find_number_in2_d_array(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.is_empty() || matrix[0].is_empty() {
            return false;
        }
        let mut i = 0;
        let mut j = matrix[0].len() - 1;
        while i < matrix.len() && j >= 0 && j < matrix[0].len() {
            if matrix[i][j] > target {
                j -= 1;
            } else if matrix[i][j] < target {
                i += 1;
            } else {
                return true;
            }
        }
        false
    }
}
