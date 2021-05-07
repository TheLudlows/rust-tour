use crate::Solution;

impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut path = vec![];
        let mut ret = vec![];
        combine_3(k as usize, n, &mut path, &mut ret, 1);
        ret
    }
}

pub fn combine_3(k: usize, n: i32, path: &mut Vec<i32>, ret: &mut Vec<Vec<i32>>, start: i32) {
    if n < 0 || path.len() > k {
        return;
    }

    if n == 0 && path.len() == k {
        ret.push(path.clone());
        return
    }

    for i in start..=9 {
        path.push(i);
        combine_3(k, n - i, path, ret, i + 1);
        path.pop();
    }
}