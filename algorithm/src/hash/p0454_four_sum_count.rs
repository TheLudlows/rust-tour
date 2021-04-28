use crate::Solution;

impl Solution {
    pub fn four_sum_count(a: Vec<i32>, b: Vec<i32>, c: Vec<i32>, d: Vec<i32>) -> i32 {
        let mut counter = std::collections::HashMap::new();
        for a in &a {
            for b in &b {
                *counter.entry(a + b).or_default() += 1;
            }
        }
        let mut res = 0;
        for c in &c {
            for d in &d {
                res += counter.get(&(-c - d)).unwrap_or(&0);
            }
        }
        res
    }
}
