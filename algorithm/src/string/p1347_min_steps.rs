use crate::Solution;

impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let mut index = vec![0; 26];
        let mut count = 0;
        s.as_bytes().iter().for_each(|b| index[(b - b'a') as usize] += 1);

        for b in t.as_bytes() {
            let i = (b - b'a') as usize;
            if index[i] > 0 {
                index[i] -= 1
            } else {
                count += 1;
            }
        }
        count
    }
}