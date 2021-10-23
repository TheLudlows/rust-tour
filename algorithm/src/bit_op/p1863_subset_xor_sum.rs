use crate::Solution;

impl Solution {
    pub fn subset_xor_sum(mut nums: Vec<i32>) -> i32 {
        let mut ret = 0;
        let mut cur = Vec::new();
        dfs(&mut nums, &mut cur, 0, &mut ret);
        ret
    }
}
pub fn dfs (num: &Vec<i32>, cur:&mut Vec<i32>, idx : usize, ret: &mut i32) {
    if cur.len() != 0 {
        let mut res = 0;
        for i in cur.iter() {
            res ^= *i;
        }
        println!("{:?}", cur);
        *ret += res;
    }

    for i in idx..num.len() {
        cur.push(num[i]);
        dfs(num, cur, i+1, ret);
        cur.pop();
    }
}
#[test]
fn test() {
    let ret = Solution::subset_xor_sum(vec![5,1,6]);
    println!("{}", ret);
}