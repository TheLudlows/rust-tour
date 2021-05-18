use std::collections::HashMap;
use crate::Solution;

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        if  n == 1 {
            return 1;
        }
        if trust.len() == 0  {
            return -1;
        }
        let mut map = HashMap::new();
        for v in trust {
            let mut vec = map.entry(v[0]).or_insert(Vec::new());
            vec.push(v[1]);
        }
        let mut judge = -1;
        for i in 1..=n {
            if !map.contains_key(&i) {
                judge = i;
            }
        }
        if judge == -1 {
            return -1;
        }
        for i in 1..=n {
            if map.contains_key(&i) && !map.get(&i).unwrap().contains(&judge) {
                return -1
            }
        }
        judge
    }
}
#[test]
fn test() {
    let r = Solution::find_judge(2,vec![vec![1,2]]);
    println!("{}", r)
}