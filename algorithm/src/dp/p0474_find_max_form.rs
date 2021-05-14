use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let mut counts = vec![];
        for s in strs {
            let mut one = 0;
            let mut zero = 0;
            for c in s.chars() {
                if c == '0' {
                    zero += 1;
                } else {
                    one += 1;
                }
            }
            counts.push((zero, one));
        }
        let mut memo = HashMap::new();
        find(&counts, m, n, counts.len(), &mut memo)
    }
}

fn find(counts: &Vec<(i32, i32)>, m: i32, n: i32, idx: usize, memo: &mut HashMap<(i32, i32, usize), i32>) -> i32 {
    if idx == 0 {
        return 0;
    }
    if memo.contains_key(&(m, n, idx)) {
        return *memo.get(&(m, n, idx)).unwrap()
    }
    let mut ret = find(counts, m, n, idx - 1, memo);
    if m >= counts[idx - 1].0 && n >= counts[idx - 1].1 {
        ret = std::cmp::max(1 + find(counts, m - counts[idx - 1].0, n - counts[idx - 1].1, idx - 1, memo), ret);
    }
    memo.insert((m, n, idx), ret);
    ret
}

#[test]
fn test() {
    let strs = vec!["10".to_string(), "0001".to_string(), "111001".to_string(), "1".to_string(), "0".to_string()];
    let r = Solution::find_max_form(strs, 5, 3);
    println!("{}", r);
}