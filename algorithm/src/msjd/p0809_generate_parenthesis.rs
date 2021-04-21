use crate::Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut v = vec![];
        dfs(0, 0, &mut String::new(), n, &mut v);
        v
    }
}

fn dfs(left: i32, right: i32, cur: &mut String, n: i32, v: &mut Vec<String>) {
    println!("{}", cur);
    if left == right && left == n {
        v.push(cur.clone());
        return;
    }
    if left < n {
        cur.push('(');
        dfs(left + 1, right, cur, n, v);
        cur.pop();
    }
    if right < left {
        cur.push(')');
        dfs(left, right + 1, cur, n, v);
        cur.pop();
    }
}

#[test]
fn test() {
    let r = Solution::generate_parenthesis(2);
    println!("{:?}", r)
}