use crate::Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s1.len() > s2.len() {
            return false;
        }
        let s = s1.as_bytes();
        let t = s2.as_bytes();
        let mut cnt = vec![0; 128];
        let mut window_cnt = vec![0; 128];
        let n = s.len();
        for i in 0..s.len() {
            window_cnt[t[i] as usize] += 1;
            cnt[s[i] as usize] += 1;
        }
        if window_cnt == cnt {
            return true;
        }

        for i in n..t.len() {
            window_cnt[t[i] as usize] +=1;
            window_cnt[t[i-n] as usize] -=1;
            if window_cnt == cnt {
                return true;
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