use crate::Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.is_empty() {
            return false;
        }
        if matrix[0].is_empty() {
            return false;
        }
        let x = matrix.len();
        let y = matrix[0].len();
        let mut l = 0;
        let mut r = x;
        let mut mid;
        while l < r {
            mid = (l + r) / 2;
            if matrix[mid][0] > target {
                r = mid;
            } else if matrix[mid][y - 1] < target {
                l = mid + 1;
            } else {
                // in this vec
                let v = &matrix[mid];
                l = 0;
                r = y;
                while l < r {
                    mid = (l + r) / 2;
                    if v[mid] > target {
                        r = mid;
                    } else if v[mid] < target {
                        l = mid + 1;
                    } else {
                        return true;
                    }
                }
            }
        }
        false
    }
    pub fn search_matrix2(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.len() == 0 {
            return false;
        }

        let m = matrix.len();
        let n = matrix[0].len();
        let mut l = 0;
        let mut r = m * n;

        while l < r {
            let mid = (l + r) / 2;
            let row = mid / n;
            let col = mid % n;

            if target == matrix[row][col] {
                return true;
            } else if target < matrix[row][col] {
                r = mid;
            } else {
                l = mid + 1;
            }
        }

        false
    }
}

#[test]
fn test() {}
