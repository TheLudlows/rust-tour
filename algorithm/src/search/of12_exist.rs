use crate::Solution;

// 深度优先， 标记数组防止重复访问
impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let chars = word.chars().collect::<Vec<char>>();
        let mut visit = vec![vec![false; board[0].len()]; board.len()];
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if find(&board, i, j, 0, &chars, &mut visit) {
                    return true;
                }
            }
        }
        false
    }
}

pub fn find(board: &Vec<Vec<char>>, i: usize, j: usize, idx: usize, chars: &Vec<char>, visit: &mut Vec<Vec<bool>>) -> bool {
    if i >= board.len() || j >= board[0].len() || visit[i][j] || board[i][j] != chars[idx] {
        return false;
    }
    if idx == chars.len() - 1 {
        return true;
    }
    visit[i][j] = true;
    let find = find(board, i + 1, j, idx + 1, chars, visit) ||
        if i > 0 { find(board, i - 1, j, idx + 1, chars, visit) } else { false } ||
        find(board, i, j + 1, idx + 1, chars, visit) ||
        if j > 0 { find(board, i, j - 1, idx + 1, chars, visit) } else { false };
    visit[i][j] = false;

    return find;
}

#[test]
fn test() {
    let r = Solution::exist(vec![vec!['a']], String::from("a"));
    println!("{}", r)
}