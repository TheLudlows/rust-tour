use crate::Solution;

impl Solution {
    pub fn permutation(s: String) -> Vec<String> {
        let mut res = vec![];
        backtrace(&s.chars().collect(), &m vec![false;s.len()], &mut String::new(),&mut res);
        res
    }
}

fn backtrace(s: &Vec<char>, vis: &mut Vec<bool>, cur: &mut String, res: &mut Vec<String>) {
    if cur.len() == s.len() {
        res.push(cur.clone())
    }
    for i in 0..s.len() {
        if !vis[i] {
            cur.push(s[i]);
            vis[i] = true;
            backtrace(s, vis, cur, res);
            vis[i] = false;
            cur.pop();
        }
    }
}