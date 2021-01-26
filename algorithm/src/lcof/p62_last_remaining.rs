use crate::Solution;

impl Solution {
    pub fn last_remaining(n: i32, m: i32) -> i32 {
        f(n,m)
    }
}
fn f(n:i32, m:i32) -> i32 {
    if n == 1 {
        return 0;
    }
    (f(n-1,m) + m) % n
}