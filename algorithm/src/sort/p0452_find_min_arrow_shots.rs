use crate::Solution;
use std::cmp::{Ordering, max, min};

impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_by(|a,b|-> Ordering {
           let ord = a[0].cmp(&b[0]);
            if ord ==  Ordering::Equal {
                a[1].cmp(&b[1])
            } else{
                ord
            }
        });

        let mut result = vec![];
        for i in 0..points.len() {
            let v = points[i].clone();
            if result.is_empty() {
                result.push(v);
            } else {
                let mut  last = result.last_mut().unwrap();
                if last[1] < v[0] {
                    result.push(v);
                }else { // 更新范围
                    last[0] = max(last[0],v[0]);
                    last[1] = min(last[1],v[1]);
                }
            }
        }
        result.len() as i32
    }
}
#[test]
fn test() {

}