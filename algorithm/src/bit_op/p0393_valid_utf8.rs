use crate::Solution;

impl Solution {
    pub fn valid_utf8(data: Vec<i32>) -> bool {
        let mut idx = 0;
        let mut n = 0;
        while idx < data.len() {
            n = data[idx];
            if n >> 7 == 0 {
                idx += 1;
            } else if n >> 5 == 6 {
                if idx + 2 > data.len() || !satisfies(&data[idx + 1..idx + 2]) {
                    return false;
                }
                idx += 2;
            } else if n >> 4 == 14 {
                if idx + 3 > data.len() || !satisfies(&data[idx + 1..idx + 3]) {
                    return false;
                }
                idx += 3;
            } else if n >> 3 == 30 {
                if idx + 4 > data.len() || !satisfies(&data[idx + 1..idx + 4]) {
                    return false;
                }
                idx += 4;
            } else {
                return false;
            }
        }
        true
    }
}

pub fn satisfies(slice: &[i32]) -> bool {
    for i in slice {
        if *i >> 6 != 2 {
            return false;
        }
    }
    true
}

#[test]
fn test() {
    let v = vec![1, 2, 3];
    let s = &v[1..3];
    println!("{:?}", s);
    let r = Solution::valid_utf8(vec![197, 130, 1]);
    println!("{}", r)
}