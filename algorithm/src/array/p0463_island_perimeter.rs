use crate::Solution;

/// 直接遍历数组，只要前面有相邻的方格，就-2
impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut count = 0;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    count += 4;
                    if j > 0 && grid[i][j - 1] == 1 {
                        count -= 2;
                    }
                    if i > 0 && grid[i - 1][j] == 1 {
                        count -= 2;
                    }
                }
            }
        }
        count
    }
}
