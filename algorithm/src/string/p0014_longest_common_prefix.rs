use crate::Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut res = String::new();
        let chars = strs.iter().map(|s| s.chars().collect()).collect::<Vec<Vec<char>>>();
        if chars.len() == 0 || chars[0].len() == 0 {
            return res;
        }
        let mut i = 0;

        loop {
            let mut same = true;
            for j in 1..chars.len() {
                if i >= chars[0].len() || i >= chars[j].len() || chars[j][i] != chars[0][i] {
                    same = false;
                    break;
                }
            }
            if same && i < chars[0].len() {
                res.push(chars[0][i] as char)
            } else {
                break;
            }
            i += 1;
        }
        res
    }
}

#[test]
fn test() {
    let v = vec!["abc".to_string(), "a".to_string()];
    let r = Solution::longest_common_prefix(v);
    println!("{}", r)
}
