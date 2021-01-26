use crate::Solution;

impl Solution {
    pub fn permutation(mut s: String) -> Vec<String> {
        let mut res = vec![];
        let mut s = s.chars().collect::<Vec<char>>();
        s.sort();
        backtrace(&s, &mut vec![false;s.len()], &mut String::new(),&mut res);
        res
    }
}

fn backtrace(s: &Vec<char>, vis: &mut Vec<bool>, cur: &mut String, res: &mut Vec<String>) {
    if cur.len() == s.len() {
        res.push(cur.clone())
    }
    for i in 0..s.len() {
        if !vis[i] {
            if i > 0 && s[i] == s[i-1] && !vis[i-1]{
                continue;
            }
            cur.push(s[i]);
            vis[i] = true;
            backtrace(s, vis, cur, res);
            vis[i] = false;
            cur.pop();
        }
    }
}