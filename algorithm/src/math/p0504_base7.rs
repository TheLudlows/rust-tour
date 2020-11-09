use crate::Solution;

impl Solution {
    pub fn convert_to_base7(mut num: i32) -> String {
        if num == 0 {
            return String::from(0 as char);
        }
        let mut v = Vec::new();
        let mut r = String::new();
        if  num < 0 {
            r.push('-');
        }
        num = num.abs();
        while num > 0 {
            let i = num % 7;
            num /=7;
            v.push(i.to_string());
        }
        v.reverse();
        v.iter().for_each(|s|r.push_str(s));
        r
    }
}