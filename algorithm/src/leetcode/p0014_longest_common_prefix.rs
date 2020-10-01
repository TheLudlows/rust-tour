use std::str::Chars;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut res = String::new();
        if strs.len() == 0 {
            return res;
        }
        let bytes = strs.iter().map(|s| s.as_bytes()).collect::<Vec<&[u8]>>();
        //println!("{}", bytes.len());
        let mut i = 0;

        loop {
            let mut same = true;
            for j in 1..bytes.len() {
                if bytes[0].len() == 0 || bytes[j].len() == 0 {
                    same = false;
                    break;
                }
                if i >= bytes[0].len() || i >= bytes[j].len() || bytes[j][i] != bytes[0][i] {
                    same = false;
                    break;
                }
            }
            if same && bytes[0].len() > i {
                res.push(bytes[0][i] as char)
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