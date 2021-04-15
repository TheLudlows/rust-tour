use crate::Solution;

impl Solution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut lands = 0;
        if grid.len() == 0 || grid[0].len() == 0 {
            return 0;
        }
        let mut max_area = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    lands += 1;
                    max_area = std::cmp::max(max_area, dfs(&mut grid, i, j));
                }
            }
        }
        max_area
    }
}

fn dfs(arr: &mut Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
    if i == arr.len() || j == arr[0].len() || arr[i][j] == 0 {
        return 0;
    }

    arr[i][j] = 0;
    return 1 +
        dfs(arr, i + 1, j) +
        if i > 0 {
            dfs(arr, i - 1, j)
        } else {
            0
        } +
        dfs(arr, i, j + 1) +
        if j > 0 {
            dfs(arr, i, j - 1)
        } else {
            0
        };
}