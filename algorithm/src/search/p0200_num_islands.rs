use crate::Solution;

// 更改相邻所有1为0，深度优先, rust usize负数处理
impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut lands = 0;
        if grid.len() == 0 || grid[0].len() == 0 {
            return 0;
        }

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == '1' {
                    lands += 1;
                    dfs(&mut grid, i, j);
                }
            }
        }
        lands
    }
}

fn dfs(arr: &mut Vec<Vec<char>>, i: usize, j: usize) {
    if i == arr.len() || j == arr[0].len() || arr[i][j] == '0' {
        return;
    }

    arr[i][j] = '0';
    dfs(arr, i + 1, j);
    if i > 0 {
        dfs(arr, i - 1, j);
    }
    dfs(arr, i, j + 1);
    if j > 0 {
        dfs(arr, i, j - 1);
    }
}