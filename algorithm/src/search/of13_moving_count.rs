use crate::Solution;

//dfs
impl Solution {
    pub fn moving_count(m: i32, n: i32, k: i32) -> i32 {
        let mut visit = vec![vec![false; n as usize]; m as usize];
        count(0, 0, k, &mut visit)
    }
}

fn count(i: usize, j: usize, k: i32, visit: &mut Vec<Vec<bool>>) -> i32 {
    if i >= visit.len() || j >= visit[0].len() || visit[i][j] || !arrive(i, j, k) {
        return 0;
    }
    visit[i][j] = true;
    1 + count(i + 1, j, k, visit) + count(i, j + 1, k, visit)
        + if i > 0 { count(i - 1, j, k, visit) } else { 0 } + if j > 0 { count(i, j - 1, k, visit) } else { 0 }
}

fn arrive(mut x: usize, mut y: usize, k: i32) -> bool {
    let (mut _x, mut _y) = (0, 0);
    while x != 0 {
        _x += x % 10;
        x = x / 10;
    }

    while y != 0 {
        _y += y % 10;
        y = y / 10;
    }
    _x + _y <= k as usize
}