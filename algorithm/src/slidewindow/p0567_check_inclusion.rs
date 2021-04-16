use crate::Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let s = s1.as_bytes();
        let t = s2.as_bytes();
        let mut cnt = vec![0; 128];
        for b in s {
            cnt[*b as usize] += 1;
        }
        for left in 0..t.len(){
            let mut right = left;
            let mut matches = 0;
            let mut window_cnt = vec![0; 128];
            while right < t.len() {
                let idx = t[right] as usize;
                if cnt[idx] == 0 {
                    break;
                }
                if window_cnt[idx] >= cnt[idx] {
                    break;
                }
                window_cnt[idx] +=1;
                matches +=1;
                if matches == s.len() {
                    return true;
                }
                right += 1;
            }
        }
        false
    }
}
#[test]
fn test() {
    let r = Solution::check_inclusion("hello".to_owned(), "ooolleoooleh".to_owned());
    println!("{}",r)
}