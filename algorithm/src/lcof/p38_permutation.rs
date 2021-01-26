use crate::Solution;
use std::iter::Enumerate;
use std::str::Chars;

impl Solution {
    pub fn permutation_(s: String) -> Vec<String> {
        let mut result = vec![];
        let mut vis = vec![false;s.len()];
        let mut chars = s.chars().collect::<Vec<char>>();
        chars.sort();
        trace(&chars,&mut String::new(),&mut vis,&mut result);
        result
    }
}
fn trace(chars:&Vec<char>,cur :&mut String,vis:&mut Vec<bool>,result:&mut Vec<String>) {
    if cur.len() == chars.len() {
        result.push(cur.clone());
    }

    for i in 0..chars.len() {
        if !vis[i] {
            if i > 0 && chars[i] == chars[i-1] && vis[i-1] {
                continue
            }
            vis[i] = true;
            cur.push(chars[i]);
            trace(chars,cur,vis,result);
            cur.pop();
            vis[i] = false;
        }
    }
}

#[test]
fn test() {
    let s = String::from("abcd");
    let chars= s.chars();
    for (i,c) in chars.into_iter().enumerate() {
        println!("{}",c)
    }
}