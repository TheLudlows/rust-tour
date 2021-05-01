use crate::Solution;
use std::str::FromStr;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack:Vec<String> = vec![];
        let mut ret  = 0;
        for t in tokens {
            if str::parse::<i32>(&t).is_ok() {
                let x = i32::from_str(&stack.pop().unwrap()).unwrap();
                let y = i32::from_str(&stack.pop().unwrap()).unwrap();
                if t.eq("+") {
                    ret = x + y;
                } else if t.eq("*") {
                    ret = x * y;
                } else if t.eq("-") {
                    ret = x - y;
                }else {
                    ret = x /y;
                }
                stack.push(ret.to_string());
            } else {
                stack.push(t);
            }
        }
        if !stack.is_empty() {
            ret = i32::from_str(&stack.pop().unwrap()).unwrap();
        }
        ret
    }
}
pub fn is_op(s: &String) -> bool {
    s.eq("+") || s.eq("*") || s.eq("-") || s.eq("/")
}