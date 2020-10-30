use crate::Solution;

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut cur = vec![];
        trace(&mut result, &mut cur, n as usize, k, 1);
        result
    }
}

fn trace(result: &mut Vec<Vec<i32>>, cur: &mut Vec<i32>, n: usize, k: i32, start: usize) {
    if cur.len() == k as usize {
        result.push(cur.clone());
        return;
    }
    for i in start..=n {
        cur.push(i as i32);
        trace(result, cur, n, k, i + 1);
        cur.pop();
    }
}

#[test]
fn test() {}
