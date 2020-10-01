

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        if p.is_empty() {
            return false;
        }
        let source = &s.as_bytes();
        let patten = &p.as_bytes();
        let (mut i, mut j) = (0, 0);
        while i < s.len() || j < patten.len() {
            if source[i] == patten[j] {
                i += 1;
                j += 1;
            } else {
                if patten[j] == '.' as u8 {
                    i += 1;
                    j += 1;
                } else if patten[j] == '*' as u8 {
                    if patten[j - 1] == source[i] {
                        i += 1;
                    }
                } else {
                    return false;
                }
            }
        }
        println!("{},{}", i, j);
        true
    }
}

#[test]
fn test() {
    let r = Solution::is_match("aa".to_string(), "a*aa".to_string());
    println!("{}", r);
}