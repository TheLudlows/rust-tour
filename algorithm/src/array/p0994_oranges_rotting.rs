use crate::Solution;
use std::collections::VecDeque;

impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut mess = VecDeque::new();
        let x = grid.len();
        let y = grid[0].len();
        let mut count = 0;
        for i in 0..x {
            for j in 0..y {
                if grid[i][j] == 2 {
                    mess.push_back((i, j));
                } else if grid[i][j] == 1 {
                    count += 1;
                }
            }
        }
        let mut round = 0;
        while !mess.is_empty() && count > 0 {
            round += 1;
            let len = mess.len();
            for _ in 0..len {
                let (m, n) = mess.pop_front().unwrap();
                if m > 0 && grid[m - 1][n] == 1 {
                    // top
                    grid[m - 1][n] = 2;
                    count -= 1;
                    mess.push_back((m - 1, n));
                }
                if n > 0 && grid[m][n - 1] == 1 {
                    // left
                    grid[m][n - 1] = 2;
                    count -= 1;
                    mess.push_back((m, n - 1));
                }
                if m + 1 < x && grid[m + 1][n] == 1 {
                    // bottom
                    grid[m + 1][n] = 2;
                    count -= 1;
                    mess.push_back((m + 1, n));
                }
                if n + 1 < y && grid[m][n + 1] == 1 {
                    // right
                    grid[m][n + 1] = 2;
                    count -= 1;
                    mess.push_back((m, n + 1));
                }
            }
        }
        if count > 0 {
            return -1;
        } else {
            round
        }
    }
}

#[test]
fn test() {
    let mut v = vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]];
    Solution::oranges_rotting(v);
}
