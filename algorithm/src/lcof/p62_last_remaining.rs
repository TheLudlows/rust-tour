use crate::Solution;

impl Solution {
    pub fn last_remaining(n: i32, m: i32) -> i32 {
        f(n, m)
    }

    pub fn last_remaining1(n: i32, m: i32) -> i32 {
        let m = m as usize;
        let mut v = (0..n).into_iter().collect::<Vec<i32>>();
        let mut i = 0;
        while v.len() > 1 {
            i = (i + m - 1) % v.len();
            v.remove(i);
        }
        v[0]
    }
}

fn f(n: i32, m: i32) -> i32 {
    if n == 1 {
        return 0;
    }
    (f(n - 1, m) + m) % n
}