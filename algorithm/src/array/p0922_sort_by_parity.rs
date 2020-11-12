use crate::Solution;

impl Solution {
    pub fn sort_array_by_parity_ii(mut a: Vec<i32>) -> Vec<i32> {
        let mut i = 0;
        let mut j = 1;
        while i < a.len() && j < a.len() {
            if a[i] % 2 == 0 {
                i += 2;
            } else if a[j] % 2 == 1 {
                j += 2;
            } else {
                a.swap(i, j);
            }
        }
        a
    }
}