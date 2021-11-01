use crate::Solution;

// 前缀和异或
impl Solution {
    pub fn xor_queries(mut arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ret = vec![0; queries.len()];
        let mut prefix_xor = vec![0; arr.len()];
        prefix_xor[0] = arr[0];
        for i in 1..arr.len() {
            prefix_xor[i] = arr[i] ^ prefix_xor[i - 1];
        }

        for i in 0..queries.len() {
            ret[i] = query(&prefix_xor, &queries[i]);
        }
        ret
    }
}

pub fn query(prefix: &Vec<i32>, query: &Vec<i32>) -> i32 {
    let start = query[0] as usize;
    let end = query[1] as usize;
    if start == 0 {
        prefix[end]
    } else {
        prefix[end] ^ prefix[start - 1]
    }
}

#[test]
fn test() {}