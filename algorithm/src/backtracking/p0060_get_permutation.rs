use crate::Solution;

impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let mut result: Vec<Vec<usize>> = vec![];
        let mut arr: Vec<usize> = vec![];
        let mut vis = vec![false; n as usize + 1];
        backtrace(n as usize, &mut arr, &mut result, &mut vis);
        result[k as usize].iter().map(|&n| n.to_string()).collect()
    }
}
fn backtrace(n:usize, arr: &mut Vec<usize>, result: &mut Vec<Vec<usize>>, vis: &mut Vec<bool>) {
    if arr.len() == n {
        result.push(arr.clone());
    } else {
        for i in 1..=n {
            if !vis[i] {
                vis[i] = true;
                arr.push(i);
                backtrace(n, arr, result, vis);
                vis[i] = false;
                arr.pop();
            }
        }
    }
}
#[test]
fn test() {

}
