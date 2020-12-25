use crate::Solution;

impl Solution {
    pub fn is_unique(astr: String) -> bool {
        let mut v = vec![0; 26];
        for b in astr.as_bytes() {
            let i = (b - b'a') as usize;
            if v[i] == 0 {
                v[i] += 1;
            } else {
                return false;
            }
        }
        return true;
    }
}

#[test]
fn test() {

}