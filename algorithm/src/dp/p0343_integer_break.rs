use crate::Solution;
use std::cmp::max;

impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        let n = n as usize;
        let mut memo = vec![0;n+1];
        int_break(n, &mut memo) as i32
    }
}
fn int_break(n:usize, memo:&mut Vec<usize>) -> usize {
    if n == 1 {
       return 1;
    }
    if memo[n] != 0 {
        return memo[n];
    }
    let mut  ret = 0;
    for i in 1..n {
        ret = max3(i*(n-i), ret, i*int_break(n-i, memo));
    }
    memo[n] = ret;
    ret
}

fn max3(v1:usize,v2:usize, v3:usize) -> usize {
    max(max(v1,v2),v3)
}
#[test]
fn test() {
    let r = Solution::integer_break(3);
    println!("{}", r);
}