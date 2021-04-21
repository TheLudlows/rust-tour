use crate::Solution;

impl Solution {
    pub fn add_binary(mut a: String, mut b: String) -> String {
        let mut pre = 0;
        let mut ret = String::new();
        loop {
            let n1 = a.pop();
            let n2 = b.pop();
            if n1.is_none() && n2.is_none() {
                break;
            }
            if let Some(x) = n1 {
                pre += x.to_digit(2).unwrap();
            }
            if let Some(x) = n2 {
                pre += x.to_digit(2).unwrap();
            }
            if pre == 0 {
                ret.push_str("0");
            } else if pre == 1 {
                pre = 0;
                ret.push_str("1");
            } else if pre == 2 {
                pre = 1;
                ret.push_str("0");
            } else {
                pre = 1;
                ret.push_str("1");
            }
        }
        if pre == 1 {
            ret.push_str("1");
        }
        ret.chars().into_iter().rev().collect()
    }
}