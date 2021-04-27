use crate::Solution;

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let mut vec = vec![];
        if p.len() > s.len() {
            return vec;
        }
        let s = s.as_bytes();
        let p = p.as_bytes();
        let mut cnt = vec![0; 128];
        let mut window_cnt = vec![0; 128];
        let n = p.len();
        for i in 0..p.len() {
            window_cnt[s[i] as usize] += 1;
            cnt[p[i] as usize] += 1;
        }
        if window_cnt == cnt {
            vec.push(0);
        }

        for i in n..s.len() {
            window_cnt[s[i] as usize] += 1;
            window_cnt[s[i - n] as usize] -= 1;
            if window_cnt == cnt {
                vec.push((i - n + 1) as i32);
            }
        }
        vec
    }
}
#[test]
fn test() {
   let r = Solution::find_anagrams("baa".to_owned(), "aa".to_owned());
    println!("{:?}",r)
}