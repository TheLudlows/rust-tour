use crate::Solution;

impl Solution {
    pub fn sorted_squares(mut a: Vec<i32>) -> Vec<i32> {
        let mut i = 0;
        let mut j = a.len() - 1;
        let mut pos = j;
        let mut res = vec![-1; a.len()];
        for id in 0..a.len() {
            a[id] = a[id].pow(2);
        }
        while i < j {
            if a[i] > a[j] {
                res[pos] = a[i];
                i += 1;
            } else {
                res[pos] = a[j];
                j -= 1;
            }
            pos -= 1;
        }
        res[pos] = a[i].pow(2);
        res
    }
}
