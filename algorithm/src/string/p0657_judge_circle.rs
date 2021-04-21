use crate::Solution;

impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let mut x = 0;
        let mut y = 0;
        moves.as_bytes().iter().for_each(|c| {
            match c {
                b'L' => x -= 1,
                b'R' => x += 1,
                b'U' => y += 1,
                b'D' => y -= 1,
                _ => ()
            }
        });
        x == 0 && y == 0
    }
}