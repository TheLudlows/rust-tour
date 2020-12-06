use crate::Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];

        for n in 1..=num_rows as usize{
            let mut row = vec![1; n];
            res.last().map(|pre| {
                for i in 1..pre.len() {
                    row[i] = pre[i] + pre[i - 1];
                }
            });
            res.push(row);
        }
        res
    }
}

#[test]
fn test() {
    println!("{:?}", Solution::generate(5));
}