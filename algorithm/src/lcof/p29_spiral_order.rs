use crate::Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = vec![];
        if matrix.is_empty() {
            return result;
        }
        let m = matrix.len();
        let n = matrix[0].len();

        let mut top = 0;
        let mut bottom = m;
        let mut left = 0;
        let mut right = n;

        while top < bottom && left < right {
            for i in left..right {
                result.push(matrix[top][i]);
            }
            top += 1;

            for i in top..bottom {
                result.push(matrix[i][right - 1]);
            }
            right -= 1;
            if top >= bottom {
                break;
            }
            for i in (left..right).rev() {
                result.push(matrix[bottom - 1][i]);
            }
            bottom -= 1;
            if left >= right {
                break;
            }
            for i in (top..bottom).rev() {
                result.push(matrix[i][left]);
            }
            left += 1;
        }
        result
    }
}

#[test]
fn test() {
    let v = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
    Solution::spiral_order(v);
}