use crate::Solution;

impl Solution {
    pub fn compress_string(s: String) -> String {
        let mut result = String::new();
        let mut chars = s.chars();
        let mut last = chars.next().unwrap();
        let mut count = 1;
        result.push(last);
        for c in chars {
            if c == last {
                count += 1;
            } else {
                last = c;
                result.push_str(count.to_string().as_str());
                result.push(c);
                count = 1;
            }
        }
        result.push_str(count.to_string().as_str());
        if result.len() > s.len() {
            s
        } else {
            result
        }
    }
}